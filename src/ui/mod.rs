pub mod components;

use self::components::*;
use crate::fissures::*;
use crate::notifications::background_notification_task;
use chrono::Utc;
use iced::widget::{Space, button, column, container, row, scrollable, text};
use iced::{
    Alignment, Border, Color, Element, Length, Padding, Subscription, Task, Theme, padding,
};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::watch;
use worldstate_parser::default_data_fetcher::CacheStrategy;
use worldstate_parser::{Fissure, FissureTier, MissionType};

pub struct VoidFissuresApp {
    pub client: Arc<reqwest::Client>,
    pub fissures: DataState<Vec<Fissure>>,
    pub active_filters: HashSet<FissureTier>,
    pub mission_filters: HashSet<MissionType>,
    pub steel_path_filter: SteelPathFilter,
    pub last_tick: chrono::DateTime<Utc>,
    pub last_fetch: chrono::DateTime<Utc>,
    pub subscriptions: SubscriptionState,
    pub subscription_tx: watch::Sender<SubscriptionState>,
    pub show_subscriptions: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Startup,
    Refresh,
    Tick(chrono::DateTime<Utc>),
    FissuresLoaded(Result<Vec<Fissure>, String>),
    FilterToggled(FissureTier),
    MissionFilterToggled(MissionType),
    SteelPathFilterChanged(SteelPathFilter),
    SubscriptionTierToggled(FissureTier),
    SubscriptionMissionToggled(MissionType),
    ToggleSubscriptions,
    TestAlert,
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
    MissionType::Hijack,
    MissionType::HiveSabotage,
    MissionType::InfestedSalvage,
    MissionType::Assault,
    MissionType::LegacyteHarvest,
];

impl VoidFissuresApp {
    pub fn new(
        subscription_tx: watch::Sender<SubscriptionState>,
        subscription_rx: watch::Receiver<SubscriptionState>,
    ) -> (Self, Task<Message>) {
        let config = AppConfig::load();
        let client = Arc::new(reqwest::Client::new());
        let now = Utc::now();

        // Spawn background notification task
        tokio::spawn(background_notification_task(
            client.clone(),
            subscription_rx,
        ));

        let app = Self {
            client,
            fissures: DataState::Loading,
            active_filters: config.active_filters,
            mission_filters: config.mission_filters,
            steel_path_filter: config.steel_path_filter,
            last_tick: now,
            last_fetch: now,
            subscriptions: config.subscriptions,
            subscription_tx,
            show_subscriptions: false,
        };

        (app, Task::done(Message::Startup))
    }

    fn save_config(&self) {
        AppConfig {
            active_filters: self.active_filters.clone(),
            mission_filters: self.mission_filters.clone(),
            steel_path_filter: self.steel_path_filter,
            subscriptions: self.subscriptions.clone(),
        }
        .save();
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Startup => {
                let client = self.client.clone();
                Task::perform(
                    async move {
                        let _ = worldstate_parser::default_data_fetcher::fetch_all(
                            CacheStrategy::Basic,
                        )
                        .await;
                        fetch_fissures(client).await
                    },
                    Message::FissuresLoaded,
                )
            }
            Message::Refresh => {
                self.fissures = DataState::Loading;
                Task::perform(fetch_fissures(self.client.clone()), Message::FissuresLoaded)
            }
            Message::Tick(now) => {
                self.last_tick = now;
                if let DataState::Loaded(fissures) = &mut self.fissures {
                    fissures.retain(|f| f.expiry > now);
                }
                if (now - self.last_fetch).num_seconds() >= REFRESH_INTERVAL_SECS {
                    return Task::perform(
                        fetch_fissures(self.client.clone()),
                        Message::FissuresLoaded,
                    );
                }
                Task::none()
            }
            Message::FissuresLoaded(result) => {
                self.last_fetch = Utc::now();
                match result {
                    Ok(mut data) => {
                        data.sort_by_key(|f| tier_to_int(f.tier));
                        self.fissures = DataState::Loaded(data);
                    }
                    Err(e) => self.fissures = DataState::Error(e),
                }
                Task::none()
            }
            Message::FilterToggled(tier) => {
                if self.active_filters.contains(&tier) {
                    self.active_filters.remove(&tier);
                } else {
                    self.active_filters.insert(tier);
                }
                self.save_config();
                Task::none()
            }
            Message::MissionFilterToggled(mtype) => {
                if self.mission_filters.contains(&mtype) {
                    self.mission_filters.remove(&mtype);
                } else {
                    self.mission_filters.insert(mtype);
                }
                self.save_config();
                Task::none()
            }
            Message::SteelPathFilterChanged(filter) => {
                self.steel_path_filter = filter;
                self.save_config();
                Task::none()
            }
            Message::SubscriptionTierToggled(tier) => {
                if self.subscriptions.tiers.contains(&tier) {
                    self.subscriptions.tiers.remove(&tier);
                } else {
                    self.subscriptions.tiers.insert(tier);
                }
                let _ = self.subscription_tx.send(self.subscriptions.clone());
                self.save_config();
                Task::none()
            }
            Message::SubscriptionMissionToggled(mtype) => {
                if self.subscriptions.mission_types.contains(&mtype) {
                    self.subscriptions.mission_types.remove(&mtype);
                } else {
                    self.subscriptions.mission_types.insert(mtype);
                }
                let _ = self.subscription_tx.send(self.subscriptions.clone());
                self.save_config();
                Task::none()
            }
            Message::ToggleSubscriptions => {
                self.show_subscriptions = !self.show_subscriptions;
                Task::none()
            }
            Message::TestAlert => {
                Task::perform(
                    async move {
                        let _ = notify_rust::Notification::new()
                            .summary("Warframe Hub")
                            .body("This is a test alert. Your notifications are working correctly!")
                            .show();
                    },
                    |_| Message::Tick(Utc::now()),
                )
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_secs(1)).map(|_| Message::Tick(Utc::now()))
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }

    pub fn view(&self) -> Element<'_, Message> {
        let next_refresh_secs =
            REFRESH_INTERVAL_SECS - (self.last_tick - self.last_fetch).num_seconds();
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
            ],
            Space::new().width(Length::Fill),
            row![
                button(
                    text(if self.show_subscriptions { "CLOSE SETTINGS" } else { "MANAGE ALERTS" })
                        .size(14)
                        .font(bold_font())
                )
                .padding([8, 16])
                .on_press(Message::ToggleSubscriptions)
                .style(move |_theme, _status| {
                    let active = self.show_subscriptions;
                    button::Style {
                        background: Some(if active { SOFT_GOLD } else { Color::TRANSPARENT }.into()),
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

        let mut sorted_mission_types: Vec<_> = ALL_MISSION_TYPES.to_vec();
        sorted_mission_types.sort_by_key(|m| mission_type_name(*m));

        let mut filter_content = column![
            row![
                text("FILTERS:").size(12).font(bold_font()).color(TEXT_DIM),
                Space::new().width(Length::Fixed(10.0)),
                filter_chip(
                    "LITH",
                    FissureTier::Lith,
                    &self.active_filters,
                    Message::FilterToggled
                ),
                filter_chip(
                    "MESO",
                    FissureTier::Meso,
                    &self.active_filters,
                    Message::FilterToggled
                ),
                filter_chip(
                    "NEO",
                    FissureTier::Neo,
                    &self.active_filters,
                    Message::FilterToggled
                ),
                filter_chip(
                    "AXI",
                    FissureTier::Axi,
                    &self.active_filters,
                    Message::FilterToggled
                ),
                filter_chip(
                    "REQUIEM",
                    FissureTier::Requiem,
                    &self.active_filters,
                    Message::FilterToggled
                ),
                filter_chip(
                    "OMNIA",
                    FissureTier::Omnia,
                    &self.active_filters,
                    Message::FilterToggled
                ),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            Space::new().height(Length::Fixed(12.0)),
            row![
                text("MODE:").size(12).font(bold_font()).color(TEXT_DIM),
                Space::new().width(Length::Fixed(10.0)),
                mode_chip(
                    "BOTH",
                    SteelPathFilter::Both,
                    self.steel_path_filter,
                    Message::SteelPathFilterChanged
                ),
                mode_chip(
                    "NORMAL",
                    SteelPathFilter::Normal,
                    self.steel_path_filter,
                    Message::SteelPathFilterChanged
                ),
                mode_chip(
                    "STEEL PATH",
                    SteelPathFilter::SteelPath,
                    self.steel_path_filter,
                    Message::SteelPathFilterChanged
                ),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            Space::new().height(Length::Fixed(12.0)),
            row![
                text("MISSIONS:").size(12).font(bold_font()).color(TEXT_DIM),
                Space::new().width(Length::Fixed(10.0)),
                row(sorted_mission_types
                    .iter()
                    .map(|&mtype| mission_filter_chip(
                        mtype,
                        &self.mission_filters,
                        Message::MissionFilterToggled
                    )))
                .spacing(8)
                .wrap()
                .vertical_spacing(8)
            ]
            .align_y(Alignment::Start),
        ];

        if self.show_subscriptions {
            filter_content = filter_content.push(Space::new().height(Length::Fixed(20.0))).push(
                container(column![
                    row![
                        text("NOTIFY ME ON:")
                            .size(12)
                            .font(bold_font())
                            .color(SOFT_GOLD),
                        Space::new().width(Length::Fill),
                        button(
                            text("TEST ALERT")
                                .size(10)
                                .font(bold_font())
                        )
                        .padding([4, 12])
                        .on_press(Message::TestAlert)
                        .style(move |_theme, _status| {
                            button::Style {
                                background: Some(Color::TRANSPARENT.into()),
                                text_color: SOFT_CYAN,
                                border: Border {
                                    color: SOFT_CYAN,
                                    width: 1.0,
                                    radius: 2.0.into(),
                                },
                                ..Default::default()
                            }
                        }),
                    ].align_y(Alignment::Center),
                    Space::new().height(Length::Fixed(12.0)),
                    row![
                        text("Tiers:")
                            .size(11)
                            .color(TEXT_DIM)
                            .width(Length::Fixed(70.0)),
                        row![
                            filter_chip(
                                "LITH",
                                FissureTier::Lith,
                                &self.subscriptions.tiers,
                                Message::SubscriptionTierToggled
                            ),
                            filter_chip(
                                "MESO",
                                FissureTier::Meso,
                                &self.subscriptions.tiers,
                                Message::SubscriptionTierToggled
                            ),
                            filter_chip(
                                "NEO",
                                FissureTier::Neo,
                                &self.subscriptions.tiers,
                                Message::SubscriptionTierToggled
                            ),
                            filter_chip(
                                "AXI",
                                FissureTier::Axi,
                                &self.subscriptions.tiers,
                                Message::SubscriptionTierToggled
                            ),
                            filter_chip(
                                "REQUIEM",
                                FissureTier::Requiem,
                                &self.subscriptions.tiers,
                                Message::SubscriptionTierToggled
                            ),
                            filter_chip(
                                "OMNIA",
                                FissureTier::Omnia,
                                &self.subscriptions.tiers,
                                Message::SubscriptionTierToggled
                            ),
                        ]
                        .spacing(10),
                    ]
                    .align_y(Alignment::Center),
                    Space::new().height(Length::Fixed(12.0)),
                    row![
                        text("Missions:")
                            .size(11)
                            .color(TEXT_DIM)
                            .width(Length::Fixed(70.0)),
                        row(sorted_mission_types.into_iter().map(|mtype| {
                            let active = self.subscriptions.mission_types.contains(&mtype);
                            button(
                                text(mission_type_name(mtype))
                                    .size(10)
                                    .font(bold_font())
                                    .align_x(Alignment::Center),
                            )
                            .padding([3, 10])
                            .on_press(Message::SubscriptionMissionToggled(mtype))
                            .style(move |_theme, _status| {
                                let base_bg = if active {
                                    Color {
                                        a: 0.2,
                                        ..SOFT_GOLD
                                    }
                                } else {
                                    Color {
                                        a: 0.03,
                                        ..Color::WHITE
                                    }
                                };
                                let border_color = if active {
                                    SOFT_GOLD
                                } else {
                                    Color {
                                        a: 0.1,
                                        ..Color::WHITE
                                    }
                                };
                                button::Style {
                                    background: Some(base_bg.into()),
                                    text_color: if active { Color::WHITE } else { TEXT_DIM },
                                    border: Border {
                                        color: border_color,
                                        width: 1.0,
                                        radius: 20.0.into(),
                                    },
                                    ..Default::default()
                                }
                            })
                            .into()
                        }))
                        .spacing(8)
                        .wrap()
                        .vertical_spacing(8)
                    ]
                    .align_y(Alignment::Start),
                ])
                .padding(15)
                .style(|_theme| container::Style {
                    border: Border {
                        color: Color {
                            a: 0.1,
                            ..SOFT_GOLD
                        },
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                }),
            );
        }

        let filter_bar = container(filter_content)
            .padding(Padding {
                top: 0.0,
                right: 20.0,
                bottom: 10.0,
                left: 20.0,
            });

        let content: Element<Message> = match &self.fissures {
            DataState::Loading => container(text("ANALYZING...").size(18).color(SOFT_CYAN))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
            DataState::Error(e) => container(
                text(format!("VOID INTERFERENCE: {}", e))
                    .size(18)
                    .color(ERROR_RED),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into(),
            DataState::Loaded(fissures) => {
                let filtered: Vec<_> = fissures
                    .iter()
                    .filter(|f| self.active_filters.contains(&f.tier))
                    .filter(|f| {
                        self.mission_filters.is_empty()
                            || f.node
                                .as_ref()
                                .is_some_and(|n| self.mission_filters.contains(&n.mission_type))
                    })
                    .filter(|f| match self.steel_path_filter {
                        SteelPathFilter::Normal => !f.is_steel_path,
                        SteelPathFilter::SteelPath => f.is_steel_path,
                        SteelPathFilter::Both => true,
                    })
                    .collect();

                if filtered.is_empty() {
                    container(text("NO MATCHING FISSURES").size(16).color(TEXT_DIM))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_x(Length::Fill)
                        .center_y(Length::Fill)
                        .into()
                } else {
                    let fissures_list = filtered
                        .into_iter()
                        .fold(column![].spacing(12).width(Length::Fill), |col, f| {
                            col.push(fissure_card::<Message>(f))
                        });
                    container(scrollable(
                        container(fissures_list)
                            .padding(padding::right(20))
                            .width(Length::Fill),
                    ))
                    .padding(Padding {
                        top: 0.0,
                        right: 20.0,
                        bottom: 20.0,
                        left: 20.0,
                    })
                    .into()
                }
            }
        };

        container(column![
            title_bar,
            filter_bar,
            Space::new().height(Length::Fixed(20.0)),
            content
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
