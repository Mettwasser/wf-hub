pub mod components;
pub mod images;
pub mod tab;

use std::{
    collections::HashSet,
    sync::Arc,
    time::Duration,
};

use chrono::Utc;
use iced::{
    Alignment,
    Border,
    Color,
    Element,
    Length,
    Subscription,
    Task,
    Theme,
    padding,
    task,
    widget::{
        Space,
        button,
        column,
        container,
        row,
        text,
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
    Fissure,
    FissureTier,
    MissionType,
};

use self::components::*;
use crate::{
    fissure_producer::fissure_event_producer,
    models::{
        AppConfig,
        DataState,
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
    pub fissures: DataState<Vec<Fissure>>,
    pub active_filters: HashSet<FissureTier>,
    pub mission_filters: HashSet<MissionType>,
    pub steel_path_filter: SteelPathFilter,
    pub last_fetch: chrono::DateTime<Utc>,
    pub subscriptions: SubscriptionState,
    pub subscription_tx: watch::Sender<SubscriptionState>,
    pub show_subscriptions: bool,
    pub audio_player: Arc<rodio::Player>,
    pub refresh_notifier: Arc<Notify>,
    pub current_tab: usize,
    pub volume: f32,

    pub volume_debouncer: Option<task::Handle>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Refresh,
    Tick,
    FissuresLoaded(DataState<Vec<Fissure>>),
    FilterToggled(FissureTier),
    MissionFilterToggled(MissionType),
    SteelPathFilterChanged(SteelPathFilter),
    SubscriptionTierToggled(FissureTier),
    SubscriptionMissionToggled(MissionType),
    ToggleSubscriptions,
    TestAlert,
    SwitchTab(usize),

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

        let (subscription_tx, subscription_rx) = watch::channel(SubscriptionState::default());
        let (fissure_tx, fissure_rx) = watch::channel(DataState::<Vec<Fissure>>::Loading);
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
            fissures: DataState::Loading,
            active_filters: config.active_filters,
            mission_filters: config.mission_filters,
            steel_path_filter: config.steel_path_filter,
            last_fetch: now,
            subscriptions: config.subscriptions,
            subscription_tx,
            show_subscriptions: false,
            audio_player: player.clone(),
            refresh_notifier: notify.clone(),
            current_tab: 0,
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

                if let DataState::Loaded(fissures) = &mut self.fissures {
                    fissures.retain(|f| f.expiry > now);
                }
            }
            Message::FissuresLoaded(mut data) => {
                self.last_fetch = Utc::now();

                if let DataState::Loaded(fissures) = &mut data {
                    fissures.sort_by_key(|f| tier_to_int(f.tier));
                }

                self.fissures = data;
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
            Message::SubscriptionTierToggled(tier) => {
                if self.subscriptions.tiers.contains(&tier) {
                    self.subscriptions.tiers.remove(&tier);
                } else {
                    self.subscriptions.tiers.insert(tier);
                }
                let _ = self.subscription_tx.send(self.subscriptions.clone());
                self.save_config();
            }
            Message::SubscriptionMissionToggled(mtype) => {
                if self.subscriptions.mission_types.contains(&mtype) {
                    self.subscriptions.mission_types.remove(&mtype);
                } else {
                    self.subscriptions.mission_types.insert(mtype);
                }
                let _ = self.subscription_tx.send(self.subscriptions.clone());
                self.save_config();
            }
            Message::ToggleSubscriptions => {
                self.show_subscriptions = !self.show_subscriptions;
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

        let title_bar = row![
            column![
                text("VOID FISSURES")
                    .size(32)
                    .font(bold_font())
                    .color(SOFT_GOLD),
                tab_buttons(self.current_tab)
            ]
            .spacing(8),
            Space::new().width(Length::Fill),
            row![
                button(
                    text(if self.show_subscriptions {
                        "CLOSE SETTINGS"
                    } else {
                        "MANAGE ALERTS"
                    })
                    .size(14)
                    .font(bold_font())
                )
                .padding([8, 16])
                .on_press(Message::ToggleSubscriptions)
                .style(move |_theme, _status| {
                    let active = self.show_subscriptions;
                    button::Style {
                        background: Some(
                            if active {
                                SOFT_GOLD
                            } else {
                                Color::TRANSPARENT
                            }
                            .into(),
                        ),
                        text_color: if active { Color::BLACK } else { SOFT_GOLD },
                        border: Border {
                            color: SOFT_GOLD,
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    }
                }),
                Space::new().width(Length::Fixed(10.0)),
                column![
                    button(
                        text("REFRESH")
                            .size(14)
                            .font(bold_font())
                            .align_x(Alignment::Center)
                    )
                    .padding([8, 16])
                    .on_press(Message::Refresh)
                    .style(refresh_button_style),
                    text(format!("Auto-refresh in: {}", countdown_text))
                        .size(11)
                        .color(TEXT_DIM)
                        .align_x(Alignment::End),
                ]
                .spacing(4)
                .align_x(Alignment::End)
            ]
            .align_y(Alignment::Start)
        ]
        .align_y(Alignment::Center)
        .padding(20);

        let content = match self.current_tab {
            0 => render_home(self),
            1 => render_settings(self),
            idx => unreachable!("No tab defined under {idx}"),
        };

        container(column![
            title_bar,
            container(content).padding(padding::horizontal(20))
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(BG_DARK.into()),
            ..Default::default()
        })
        .into()
    }
}
