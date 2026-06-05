pub mod components;
pub mod images;

use std::{
    collections::HashSet,
    sync::Arc,
    time::Duration,
};

use chrono::Utc;
use iced::{
    Element,
    Length,
    Subscription,
    Task,
    Theme,
    padding,
    task,
    widget::{
        column,
        container,
        row,
    },
};
use rodio::{
    DeviceSinkBuilder,
    Player,
};
use tokio::sync::{
    Notify,
    watch,
};
use worldstate_parser::{
    FissureTier,
    MissionType,
};

use self::components::*;
use crate::{
    fissure_producer::fissure_event_producer,
    models::{
        AppConfig,
        AppData,
        DataState,
        LastFetch,
        SteelPathFilter,
        SubscriptionState,
        tier_to_int,
    },
    notifications::{
        background_notification_task,
        get_source,
    },
};

pub struct VoidFissuresApp {
    pub fissures: DataState<Box<AppData>>,
    pub last_fetch: chrono::DateTime<Utc>,
    pub active_filters: HashSet<FissureTier>,
    pub mission_filters: HashSet<MissionType>,
    pub steel_path_filter: SteelPathFilter,
    pub subscriptions: SubscriptionState,
    pub subscription_tx: watch::Sender<SubscriptionState>,
    pub show_subscriptions: bool,
    pub audio_player: Arc<rodio::Player>,
    pub refresh_notifier: Arc<Notify>,
    pub current_tab: usize,
    pub selected_archimedea_tab: usize,
    pub volume: f32,

    pub volume_debouncer: Option<task::Handle>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Refresh,
    Tick,
    FissuresLoaded(DataState<Box<AppData>>),
    FilterToggled(FissureTier),
    MissionFilterToggled(MissionType),
    SteelPathFilterChanged(SteelPathFilter),

    // is_steel_path, T
    SubscriptionTierToggled(bool, FissureTier),
    SubscriptionMissionToggled(bool, MissionType),

    ToggleSubscriptions,
    ToggleNotifications(bool),
    TestAlert,
    SwitchTab(usize),
    SwitchArchimedeaTab(usize),

    // Volume
    ChangeVolume(f32),
    CommitVolumeChange,
    TestVolume,
}

const REFRESH_INTERVAL_SECS: i64 = 300;

const ALL_MISSION_TYPES: &[MissionType] = &[
    MissionType::Capture,
    MissionType::Defense,
    MissionType::Exterminate,
    MissionType::Rescue,
    MissionType::Sabotage,
    MissionType::Survival,
    MissionType::Spy,
    MissionType::Interception,
    MissionType::MobileDefense,
    MissionType::Excavation,
    MissionType::Disruption,
    MissionType::VoidFlood,
    MissionType::VoidCascade,
    MissionType::VoidArmaggedon,
    MissionType::Alchemy,
    MissionType::HiveSabotage,
    MissionType::Assault,
];

impl VoidFissuresApp {
    pub fn init() -> (Self, Task<Message>) {
        let config = AppConfig::load();
        let now = Utc::now();

        let (subscription_tx, subscription_rx) = watch::channel(config.subscriptions.clone());
        let (fissure_tx, fissure_rx) = watch::channel(DataState::<Box<AppData>>::Loading);
        let notify = Arc::new(Notify::new());

        // SAFETY: This should have a static lifetime anyway. So it's cleaned up when the app
        // closes. why? When the sink drops, the audio does not play via the player anymore.
        // So we need to prevent the handle from being dropped when it goes out of scope
        let handle = std::mem::ManuallyDrop::new(
            DeviceSinkBuilder::open_default_sink().expect("open default audio stream"),
        );
        let player = Arc::new(Player::connect_new(handle.mixer()));
        player.set_volume(config.volume);

        let app = Self {
            fissures: config
                .last_fetch
                .as_ref()
                .map(|lf| {
                    let archimedea = lf.archimedea.clone().unwrap_or({
                        worldstate_parser::ArchimedeaRoot {
                            deep: None,
                            elite_deep: None,
                            temporal: None,
                            elite_temporal: None,
                        }
                    });
                    DataState::Loaded(Box::new(AppData {
                        fissures: lf.fissures.clone(),
                        archimedea,
                    }))
                })
                .unwrap_or(DataState::Loading),
            active_filters: config.active_filters,
            mission_filters: config.mission_filters,
            steel_path_filter: config.steel_path_filter,
            last_fetch: config.last_fetch.as_ref().map(|lf| lf.at).unwrap_or(now),
            subscriptions: config.subscriptions,
            subscription_tx,
            show_subscriptions: false,
            audio_player: player.clone(),
            refresh_notifier: notify.clone(),
            current_tab: 0,
            selected_archimedea_tab: 0,
            volume: config.volume,
            volume_debouncer: None,
        };

        let fissure_stream = {
            let rx = fissure_rx.clone();
            iced::futures::stream::unfold(rx, async move |mut rx| {
                if rx.changed().await.is_ok() {
                    let data = rx.borrow().clone();
                    Some((Message::FissuresLoaded(data), rx))
                } else {
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    Some((
                        Message::FissuresLoaded(DataState::Error("Channel closed".to_owned())),
                        rx,
                    ))
                }
            })
        };

        (
            app,
            Task::batch([
                Task::stream(fissure_stream),
                Task::future(background_notification_task(
                    subscription_rx,
                    player.clone(),
                    fissure_rx.clone(),
                ))
                .discard(),
                Task::future(fissure_event_producer(fissure_tx, notify.clone())).discard(),
            ]),
        )
    }

    fn save_config(&self) {
        AppConfig {
            active_filters: self.active_filters.clone(),
            mission_filters: self.mission_filters.clone(),
            steel_path_filter: self.steel_path_filter,
            subscriptions: self.subscriptions.clone(),
            volume: self.volume,
            last_fetch: match &self.fissures {
                DataState::Loading => None,
                DataState::Loaded(data) => Some(LastFetch {
                    fissures: data.fissures.clone(),
                    archimedea: Some(data.archimedea.clone()),
                    at: self.last_fetch,
                }),
                DataState::Error(_) => None,
            },
        }
        .save();
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Refresh => {
                self.fissures = DataState::Loading;
                self.refresh_notifier.notify_one();
            }
            Message::Tick => {
                let now = Utc::now();

                if let DataState::Loaded(data) = &mut self.fissures {
                    data.fissures.retain(|f| f.expiry > now);
                }
            }
            Message::FissuresLoaded(mut data) => {
                self.last_fetch = Utc::now();

                if let DataState::Loaded(app_data) = &mut data {
                    app_data.fissures.sort_by_key(|f| tier_to_int(f.tier));
                }

                self.fissures = data;
                self.save_config();
            }
            Message::FilterToggled(tier) => {
                if self.active_filters.contains(&tier) {
                    self.active_filters.remove(&tier);
                } else {
                    self.active_filters.insert(tier);
                }
                self.save_config();
            }
            Message::MissionFilterToggled(mtype) => {
                if self.mission_filters.contains(&mtype) {
                    self.mission_filters.remove(&mtype);
                } else {
                    self.mission_filters.insert(mtype);
                }
                self.save_config();
            }
            Message::SteelPathFilterChanged(filter) => {
                self.steel_path_filter = filter;
                self.save_config();
            }
            Message::SubscriptionTierToggled(is_steel_path, tier) => {
                let sub = if is_steel_path {
                    &mut self.subscriptions.steel_path
                } else {
                    &mut self.subscriptions.normal
                };

                if sub.tiers.contains(&tier) {
                    sub.tiers.remove(&tier);
                } else {
                    sub.tiers.insert(tier);
                }
                let _ = self.subscription_tx.send(self.subscriptions.clone());
                self.save_config();
            }
            Message::SubscriptionMissionToggled(is_steel_path, mtype) => {
                let sub = if is_steel_path {
                    &mut self.subscriptions.steel_path
                } else {
                    &mut self.subscriptions.normal
                };

                if sub.mission_types.contains(&mtype) {
                    sub.mission_types.remove(&mtype);
                } else {
                    sub.mission_types.insert(mtype);
                }
                let _ = self.subscription_tx.send(self.subscriptions.clone());
                self.save_config();
            }
            Message::ToggleSubscriptions => {
                self.show_subscriptions = !self.show_subscriptions;
            }
            Message::ToggleNotifications(enabled) => {
                self.subscriptions.enabled = enabled;
                let _ = self.subscription_tx.send(self.subscriptions.clone());
                self.save_config();
            }
            Message::TestAlert => {
                let _ = notify_rust::Notification::new()
                    .summary("Warframe Hub")
                    .body("This is a test alert. Your notifications are working correctly!")
                    .show();

                self.audio_player.append(get_source());
            }
            Message::SwitchTab(new_tab_idx) => {
                self.current_tab = new_tab_idx;
            }
            Message::SwitchArchimedeaTab(new_tab_idx) => {
                self.selected_archimedea_tab = new_tab_idx;
            }
            Message::TestVolume => {
                self.audio_player.append(get_source());
            }
            Message::ChangeVolume(volume) => {
                self.volume = volume;
                self.audio_player.set_volume(volume);

                let (task, handle) =
                    Task::perform(tokio::time::sleep(Duration::from_millis(300)), |_| {
                        Message::CommitVolumeChange
                    })
                    .abortable();

                self.volume_debouncer = Some(handle.abort_on_drop());

                return task;
            }
            Message::CommitVolumeChange => {
                self.save_config();
                self.volume_debouncer = None;
            }
        };

        Task::none()
    }

    pub fn tick_subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_secs(1)).map(|_| Message::Tick)
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }

    pub fn view(&self) -> Element<'_, Message> {
        let next_refresh_secs =
            REFRESH_INTERVAL_SECS - (Utc::now() - self.last_fetch).num_seconds();
        let next_refresh_secs = next_refresh_secs.max(0);
        let countdown_text = format!(
            "{:02}:{:02}",
            next_refresh_secs / 60,
            next_refresh_secs % 60
        );

        let sidebar = render_sidebar(self);
        let header = render_header(self, &countdown_text);

        let content = match self.current_tab {
            0 => render_fissures(self),
            1 => render_archimedea(self),
            2 => render_settings(self),
            idx => unreachable!("No tab defined under {idx}"),
        };

        let right_pane = column![
            header,
            container(content)
                .padding(padding::horizontal(20))
                .height(Length::Fill)
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        container(row![sidebar, right_pane])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(BG_DARK.into()),
                ..Default::default()
            })
            .into()
    }
}
