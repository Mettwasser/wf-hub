use std::collections::HashSet;

use chrono::Utc;
use iced::{
    Alignment,
    Border,
    Color,
    Element,
    Font,
    Length,
    Theme,
    font::Weight,
    padding,
    widget::{
        Space,
        button,
        column,
        container,
        row,
        scrollable,
        slider,
        space,
        svg,
        text,
    },
};
use worldstate_parser::{
    Faction,
    Fissure,
    FissureTier,
    MissionType,
};

use crate::{
    models::{
        DataState,
        SteelPathFilter,
        mission_type_name,
    },
    ui::{
        ALL_MISSION_TYPES,
        Message,
        VoidFissuresApp,
        images::IMAGE_DIR,
    },
};

// Visual constants restored from user manual changes
pub const BG_DARK: Color = Color::from_rgb(0.1, 0.1, 0.12);
pub const CARD_BG: Color = Color::from_rgb(0.12, 0.12, 0.15);
pub const SOFT_GOLD: Color = Color::from_rgb(0.7, 0.55, 0.3);
pub const SOFT_CYAN: Color = Color::from_rgb(0.3, 0.6, 0.7);
pub const TEXT_DIM: Color = Color::from_rgb(0.6, 0.6, 0.7);
pub const ERROR_RED: Color = Color::from_rgb(0.7, 0.3, 0.3);

pub fn bold_font() -> Font {
    Font {
        weight: Weight::Bold,
        ..Font::DEFAULT
    }
}

pub fn refresh_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Hovered => button::Style {
            background: Some(
                Color {
                    a: 0.1,
                    ..SOFT_GOLD
                }
                .into(),
            ),
            text_color: SOFT_GOLD,
            border: Border {
                color: SOFT_GOLD,
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        },
        _ => button::Style {
            background: None,
            text_color: Color {
                a: 0.8,
                ..SOFT_GOLD
            },
            border: Border {
                color: Color {
                    a: 0.5,
                    ..SOFT_GOLD
                },
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        },
    }
}

pub fn filter_chip<'a, Message: Clone + 'a>(
    label: &'a str,
    tier: FissureTier,
    active_set: &HashSet<FissureTier>,
    on_press: impl Fn(FissureTier) -> Message + 'a,
) -> Element<'a, Message> {
    let active = active_set.contains(&tier);
    let color = get_tier_color(tier);

    button(
        text(label)
            .size(11)
            .font(bold_font())
            .align_x(Alignment::Center),
    )
    .padding([4, 12])
    .on_press(on_press(tier))
    .style(move |_theme, _status| {
        let base_bg = if active {
            Color { a: 0.2, ..color }
        } else {
            Color {
                a: 0.05,
                ..Color::WHITE
            }
        };

        let border_color = if active {
            color
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
}

pub fn mode_chip<'a, Message: Clone + 'a>(
    label: &'a str,
    mode: SteelPathFilter,
    active_mode: SteelPathFilter,
    on_press: impl Fn(SteelPathFilter) -> Message + 'a,
) -> Element<'a, Message> {
    let active = mode == active_mode;

    button(
        text(label)
            .size(11)
            .font(bold_font())
            .align_x(Alignment::Center),
    )
    .padding([4, 12])
    .on_press(on_press(mode))
    .style(move |_theme, _status| {
        let base_bg = if active {
            Color {
                a: 0.4,
                ..SOFT_GOLD
            }
        } else {
            Color {
                a: 0.05,
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
}

pub fn mission_filter_chip<'a, Message: Clone + 'a>(
    mtype: MissionType,
    active_set: &HashSet<MissionType>,
    on_press: impl Fn(MissionType) -> Message + 'a,
) -> Element<'a, Message> {
    let label = mission_type_name(mtype);
    let active = active_set.contains(&mtype);

    button(
        text(label)
            .size(10)
            .font(bold_font())
            .align_x(Alignment::Center),
    )
    .padding([3, 10])
    .on_press(on_press(mtype))
    .style(move |_theme, _status| {
        let base_bg = if active {
            Color {
                a: 0.2,
                ..SOFT_CYAN
            }
        } else {
            Color {
                a: 0.03,
                ..Color::WHITE
            }
        };

        let border_color = if active {
            SOFT_CYAN
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
}

pub fn faction_icon<'a, Message: Clone + 'a>(faction: Faction) -> Element<'a, Message> {
    let name = format!("{faction:?}").to_lowercase();
    let svg_path = format!("{name}.svg");

    if let Some(content) = IMAGE_DIR.get_file(svg_path).map(|file| file.contents()) {
        container(
            svg(svg::Handle::from_memory(content))
                .width(Length::Fixed(50.0))
                .height(Length::Fixed(50.0))
                .style(|_theme, _status| svg::Style {
                    color: Some(Color {
                        r: 0.7,
                        g: 0.7,
                        b: 0.7,
                        a: 1.0,
                    }),
                }),
        )
        .padding(5)
        .into()
    } else {
        Space::new().width(Length::Fixed(50.0)).into()
    }
}

pub fn fissure_card<'a, Message: Clone + 'a>(fissure: &Fissure) -> Element<'a, Message> {
    let tier_color = get_tier_color(fissure.tier);

    let tier_label = container(
        text(format!("{:?}", fissure.tier).to_uppercase())
            .size(10)
            .font(bold_font())
            .color(Color::WHITE),
    )
    .padding([2, 8])
    .style(move |_theme| container::Style {
        background: Some(
            Color {
                a: 0.2,
                ..tier_color
            }
            .into(),
        ),
        border: Border {
            color: Color {
                a: 0.5,
                ..tier_color
            },
            width: 1.0,
            radius: 2.0.into(),
        },
        ..Default::default()
    });

    let node_name = fissure
        .node
        .as_ref()
        .map(|n| n.name.clone())
        .unwrap_or_else(|| "UNKNOWN SECTOR".to_string());

    let planet = fissure
        .node
        .as_ref()
        .map(|n| n.planet.clone())
        .unwrap_or_else(|| "SYSTEM".to_string());

    let mtype = fissure
        .node
        .as_ref()
        .map(|n| mission_type_name(n.mission_type))
        .unwrap_or_else(|| "UNKNOWN MISSION".to_string());

    let faction = fissure.node.as_ref().map(|n| n.faction);

    let sp_badge = if fissure.is_steel_path {
        Some(
            container(
                row![
                    svg(svg::Handle::from_memory(
                        IMAGE_DIR.get_file("sp-logo.svg").unwrap().contents()
                    ))
                    .width(Length::Fixed(16.0))
                    .height(Length::Fixed(16.0))
                    .style(|_theme, _status| svg::Style {
                        color: Some(Color {
                            r: 0.7,
                            g: 0.7,
                            b: 0.7,
                            a: 1.0,
                        }),
                    }),
                    text("STEEL PATH")
                        .size(10)
                        .font(bold_font())
                        .color(ERROR_RED),
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            )
            .padding([2, 6])
            .style(|_theme| container::Style {
                border: Border {
                    color: Color {
                        a: 0.5,
                        ..ERROR_RED
                    },
                    width: 1.0,
                    radius: 2.0.into(),
                },
                ..Default::default()
            }),
        )
    } else {
        None
    };

    container(row![
        // Discrete Left Accent
        container(Space::new().width(Length::Fixed(3.0)))
            .height(Length::Fill)
            .style(move |_theme| container::Style {
                background: Some(tier_color.into()),
                ..Default::default()
            }),
        row![
            if let Some(f) = faction {
                faction_icon(f)
            } else {
                Space::new().width(Length::Fixed(50.0)).into()
            },
            container(
                column![
                    row![
                        tier_label,
                        Space::new().width(Length::Fixed(10.0)),
                        if let Some(badge) = sp_badge {
                            Element::from(badge)
                        } else {
                            Space::new().width(Length::Fixed(0.0)).into()
                        },
                    ]
                    .align_y(Alignment::Center),
                    Space::new().height(Length::Fixed(6.0)),
                    column![
                        text(planet.to_uppercase())
                            .size(18)
                            .font(bold_font())
                            .color(Color::WHITE),
                        text(node_name.to_uppercase()).size(14).color(TEXT_DIM),
                    ],
                ]
                .width(Length::Fixed(250.0))
            ),
            container(text(mtype).size(14).font(bold_font()).color(SOFT_GOLD))
                .width(Length::Fixed(180.0))
                .align_x(Alignment::Start),
            Space::new().width(Length::Fill),
            container(
                text(format_eta(fissure.expiry))
                    .size(15)
                    .font(bold_font())
                    .color(SOFT_CYAN)
            )
            .width(Length::Fixed(100.0))
            .align_x(Alignment::End),
        ]
        .padding(12)
        .spacing(15)
        .width(Length::Fill)
        .align_y(Alignment::Center)
    ])
    .width(Length::Fill)
    .style(move |_theme| container::Style {
        background: Some(CARD_BG.into()),
        border: Border {
            color: Color {
                a: 0.1,
                ..Color::WHITE
            },
            width: 1.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    })
    .into()
}

pub fn get_tier_color(tier: FissureTier) -> Color {
    match tier {
        FissureTier::Lith => Color::from_rgb(0.7411, 0.5686, 0.4666),
        FissureTier::Meso => Color::from_rgb(0.20, 0.48, 0.55),
        FissureTier::Neo => Color::from_rgb(0.8196, 0.8156, 0.8196),
        FissureTier::Axi => Color::from_rgb(0.9254, 0.8823, 0.4588),
        FissureTier::Requiem => Color::from_rgb(0.6, 0.2, 0.2),
        FissureTier::Omnia => Color::from_rgb(0.5, 0.3, 0.8),
    }
}

pub fn format_eta(expiry: chrono::DateTime<Utc>) -> String {
    let now = Utc::now();
    if expiry <= now {
        return "EXPIRED".to_string();
    }
    let diff = expiry - now;
    let mins = diff.num_minutes();
    let secs = diff.num_seconds() % 60;
    if mins > 60 {
        format!("{}h {}m", mins / 60, mins % 60)
    } else {
        format!("{}m {}s", mins, secs)
    }
}

pub fn render_home(app: &VoidFissuresApp) -> Element<'_, Message> {
    let mut sorted_mission_types: Vec<_> = ALL_MISSION_TYPES.to_vec();
    sorted_mission_types.sort_by_key(|m| mission_type_name(*m));

    let mut filter_content = column![
        row![
            text("FILTERS:").size(12).font(bold_font()).color(TEXT_DIM),
            Space::new().width(Length::Fixed(10.0)),
            filter_chip(
                "LITH",
                FissureTier::Lith,
                &app.active_filters,
                Message::FilterToggled
            ),
            filter_chip(
                "MESO",
                FissureTier::Meso,
                &app.active_filters,
                Message::FilterToggled
            ),
            filter_chip(
                "NEO",
                FissureTier::Neo,
                &app.active_filters,
                Message::FilterToggled
            ),
            filter_chip(
                "AXI",
                FissureTier::Axi,
                &app.active_filters,
                Message::FilterToggled
            ),
            filter_chip(
                "REQUIEM",
                FissureTier::Requiem,
                &app.active_filters,
                Message::FilterToggled
            ),
            filter_chip(
                "OMNIA",
                FissureTier::Omnia,
                &app.active_filters,
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
                app.steel_path_filter,
                Message::SteelPathFilterChanged
            ),
            mode_chip(
                "NORMAL",
                SteelPathFilter::Normal,
                app.steel_path_filter,
                Message::SteelPathFilterChanged
            ),
            mode_chip(
                "STEEL PATH",
                SteelPathFilter::SteelPath,
                app.steel_path_filter,
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
                    &app.mission_filters,
                    Message::MissionFilterToggled
                )))
            .spacing(8)
            .wrap()
            .vertical_spacing(8)
        ]
        .align_y(Alignment::Start),
    ];

    if app.show_subscriptions {
        let mut sorted_mission_types: Vec<_> = ALL_MISSION_TYPES.to_vec();
        sorted_mission_types.sort_by_key(|m| mission_type_name(*m));

        let render_sub_section = |is_sp: bool, sub: &crate::models::FissureSubscription| {
            column![
                text(if is_sp { "STEEL PATH" } else { "STAR CHART" })
                    .size(11)
                    .font(bold_font())
                    .color(SOFT_CYAN),
                Space::new().height(Length::Fixed(8.0)),
                row![
                    text("Tiers:")
                        .size(11)
                        .color(TEXT_DIM)
                        .width(Length::Fixed(70.0)),
                    row![
                        filter_chip("LITH", FissureTier::Lith, &sub.tiers, move |t| {
                            Message::SubscriptionTierToggled(is_sp, t)
                        }),
                        filter_chip("MESO", FissureTier::Meso, &sub.tiers, move |t| {
                            Message::SubscriptionTierToggled(is_sp, t)
                        }),
                        filter_chip("NEO", FissureTier::Neo, &sub.tiers, move |t| {
                            Message::SubscriptionTierToggled(is_sp, t)
                        }),
                        filter_chip("AXI", FissureTier::Axi, &sub.tiers, move |t| {
                            Message::SubscriptionTierToggled(is_sp, t)
                        }),
                        filter_chip("REQUIEM", FissureTier::Requiem, &sub.tiers, move |t| {
                            Message::SubscriptionTierToggled(is_sp, t)
                        }),
                        filter_chip("OMNIA", FissureTier::Omnia, &sub.tiers, move |t| {
                            Message::SubscriptionTierToggled(is_sp, t)
                        }),
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
                    row(sorted_mission_types.iter().map(|&mtype| {
                        let active = sub.mission_types.contains(&mtype);
                        button(
                            text(mission_type_name(mtype))
                                .size(10)
                                .font(bold_font())
                                .align_x(Alignment::Center),
                        )
                        .padding([3, 10])
                        .on_press(Message::SubscriptionMissionToggled(is_sp, mtype))
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
            ]
        };

        filter_content = filter_content
            .push(Space::new().height(Length::Fixed(20.0)))
            .push(
                container(column![
                    row![
                        text("NOTIFY ME ON:")
                            .size(12)
                            .font(bold_font())
                            .color(SOFT_GOLD),
                        Space::new().width(Length::Fill),
                        button(text("TEST ALERT").size(10).font(bold_font()))
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
                    ]
                    .align_y(Alignment::Center),
                    Space::new().height(Length::Fixed(12.0)),
                    render_sub_section(false, &app.subscriptions.normal),
                    Space::new().height(Length::Fixed(20.0)),
                    render_sub_section(true, &app.subscriptions.steel_path),
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

    let filter_bar = container(filter_content).padding(padding::bottom(10));

    let content: Element<'_, Message> = match &app.fissures {
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
        DataState::Loaded(data) => {
            let filtered: Vec<_> = data
                .fissures
                .iter()
                .filter(|f| app.active_filters.contains(&f.tier))
                .filter(|f| {
                    app.mission_filters.is_empty()
                        || f.node
                            .as_ref()
                            .is_some_and(|n| app.mission_filters.contains(&n.mission_type))
                })
                .filter(|f| match app.steel_path_filter {
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
                .padding(padding::bottom(20))
                .into()
            }
        }
    };

    column![
        filter_bar,
        Space::new().height(Length::Fixed(20.0)),
        content
    ]
    .into()
}

pub fn render_settings(app: &VoidFissuresApp) -> Element<'_, Message> {
    let slider = column![
        text("Volume"),
        row![
            row![
                slider(
                    0.0..=100.0,
                    app.volume * 100.0,
                    |val| Message::ChangeVolume(val / 100.0)
                ),
                text(format!("{:.0}%", app.volume * 100.0)),
                space::horizontal().width(8),
                button("Test")
                    .on_press(Message::TestVolume)
                    .style(|_, _| button::Style {
                        background: Some(Color::TRANSPARENT.into()),
                        text_color: SOFT_CYAN,
                        border: Border {
                            color: SOFT_CYAN,
                            width: 1.0,
                            radius: 2.0.into(),
                        },
                        ..Default::default()
                    })
            ]
            .spacing(4)
            .width(Length::FillPortion(2))
            .align_y(Alignment::Center),
            space::horizontal().width(Length::FillPortion(2))
        ]
    ]
    .spacing(4);

    column![slider,].into()
}

fn sidebar_button<'a>(
    title: &'static str,
    active: bool,
    on_press: Message,
) -> Element<'a, Message> {
    let text_color = if active { Color::WHITE } else { TEXT_DIM };

    button(
        row![
            // Active indicator bar
            container(Space::new())
                .width(Length::Fixed(4.0))
                .height(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(
                        if active {
                            SOFT_GOLD
                        } else {
                            Color::TRANSPARENT
                        }
                        .into()
                    ),
                    ..Default::default()
                }),
            Space::new().width(Length::Fixed(12.0)),
            text(title).size(13).font(bold_font()).color(text_color),
        ]
        .align_y(Alignment::Center)
        .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fixed(48.0))
    .padding(padding::right(12))
    .on_press(on_press)
    .style(move |_theme, status| {
        let bg = match status {
            button::Status::Hovered | button::Status::Pressed => Color {
                a: 0.05,
                ..Color::WHITE
            },
            _ if active => Color {
                a: 0.08,
                ..SOFT_GOLD
            },
            _ => Color::TRANSPARENT,
        };

        button::Style {
            background: Some(bg.into()),
            border: Border::default(),
            ..Default::default()
        }
    })
    .into()
}

pub fn render_sidebar(app: &VoidFissuresApp) -> Element<'_, Message> {
    let logo = container(
        row![
            text("WARFRAME")
                .size(20)
                .font(bold_font())
                .color(Color::WHITE),
            text("HUB").size(20).font(bold_font()).color(SOFT_GOLD),
        ]
        .spacing(4),
    )
    .padding(padding::bottom(20));

    let menu = column![
        sidebar_button("VOID FISSURES", app.current_tab == 0, Message::SwitchTab(0)),
        sidebar_button(
            "ELITE ARCHIMEDEA",
            app.current_tab == 1,
            Message::SwitchTab(1)
        ),
        sidebar_button("SETTINGS", app.current_tab == 2, Message::SwitchTab(2)),
    ]
    .spacing(8);

    let volume_slider = column![
        text("SYSTEM VOLUME")
            .size(10)
            .font(bold_font())
            .color(TEXT_DIM),
        row![
            text(format!("{:.0}%", app.volume * 100.0))
                .size(12)
                .font(bold_font())
                .color(Color::WHITE),
            slider(
                0.0..=100.0,
                app.volume * 100.0,
                |val| Message::ChangeVolume(val / 100.0)
            )
            .width(Length::Fill),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
    ]
    .spacing(6);

    container(
        column![logo, menu, Space::new().height(Length::Fill), volume_slider,]
            .spacing(15)
            .height(Length::Fill),
    )
    .width(Length::Fixed(220.0))
    .height(Length::Fill)
    .padding(20)
    .style(|_theme| container::Style {
        background: Some(Color::from_rgb(0.08, 0.08, 0.10).into()),
        border: Border {
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.03),
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    })
    .into()
}

pub fn render_header<'a>(app: &'a VoidFissuresApp, countdown_text: &str) -> Element<'a, Message> {
    let title = match app.current_tab {
        0 => "VOID FISSURES",
        1 => "ELITE ARCHIMEDEA",
        2 => "SETTINGS",
        _ => "WARFRAME HUB",
    };

    let title_text = text(title).size(28).font(bold_font()).color(SOFT_GOLD);

    let mut action_buttons = row![].spacing(10).align_y(Alignment::Center);

    if app.current_tab == 0 {
        action_buttons = action_buttons.push(
            button(text("MANAGE ALERTS").size(12).font(bold_font()))
                .padding([8, 16])
                .on_press(Message::ToggleSubscriptions)
                .style(move |_theme, _status| {
                    let active = app.show_subscriptions;
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
        );
    }

    if app.current_tab == 0 || app.current_tab == 1 {
        action_buttons = action_buttons.push(
            button(
                text("REFRESH")
                    .size(12)
                    .font(bold_font())
                    .align_x(Alignment::Center),
            )
            .padding([8, 16])
            .on_press(Message::Refresh)
            .style(refresh_button_style),
        );

        let actions = column![
            action_buttons,
            text(format!("Auto-refresh in: {}", countdown_text))
                .size(10)
                .color(TEXT_DIM)
                .align_x(Alignment::End),
        ]
        .spacing(4)
        .align_x(Alignment::End);

        container(
            row![title_text, Space::new().width(Length::Fill), actions].align_y(Alignment::Center),
        )
        .padding(20)
        .width(Length::Fill)
        .into()
    } else {
        container(
            row![title_text, Space::new().width(Length::Fill), action_buttons]
                .align_y(Alignment::Center),
        )
        .padding(20)
        .width(Length::Fill)
        .into()
    }
}

fn render_archimedea_mission_card<'a>(
    index: usize,
    mission: &'a worldstate_parser::ArchimedeaMission,
) -> Element<'a, Message> {
    let faction = mission.faction;
    let mtype = mission_type_name(mission.mission_type);

    let mut difficulties_content = column![].spacing(10);

    if mission.difficulties.is_empty() {
        difficulties_content = difficulties_content.push(
            column![
                text("DEVIATION")
                    .size(11)
                    .font(bold_font())
                    .color(SOFT_GOLD),
                text("Unknown")
                    .size(15)
                    .font(bold_font())
                    .color(Color::WHITE),
                text("No deviation details available")
                    .size(13)
                    .color(TEXT_DIM),
            ]
            .spacing(2),
        );

        difficulties_content = difficulties_content.push(
            column![
                text("RISK MODIFIERS")
                    .size(11)
                    .font(bold_font())
                    .color(ERROR_RED),
                column![
                    text("Unknown").size(14).font(bold_font()).color(ERROR_RED),
                    text("No risk details available").size(12).color(TEXT_DIM)
                ]
                .spacing(1)
            ]
            .spacing(4),
        );
    } else {
        for diff in &mission.difficulties {
            let dev_title = if diff.deviation.title.is_empty() {
                "Unknown"
            } else {
                &diff.deviation.title
            };

            let dev_desc = diff
                .deviation
                .description
                .as_deref()
                .filter(|s| !s.is_empty())
                .unwrap_or("No deviation description available");

            let deviation_block = column![
                text("DEVIATION")
                    .size(11)
                    .font(bold_font())
                    .color(SOFT_GOLD),
                text(dev_title)
                    .size(15)
                    .font(bold_font())
                    .color(Color::WHITE),
                text(dev_desc).size(13).color(TEXT_DIM),
            ]
            .spacing(2);

            difficulties_content = difficulties_content.push(deviation_block);

            if diff.risks.is_empty() {
                difficulties_content = difficulties_content.push(
                    column![
                        text("RISK MODIFIERS")
                            .size(11)
                            .font(bold_font())
                            .color(ERROR_RED),
                        column![
                            text("Unknown").size(14).font(bold_font()).color(ERROR_RED),
                            text("No risk details available").size(12).color(TEXT_DIM)
                        ]
                        .spacing(1)
                    ]
                    .spacing(4),
                );
            } else {
                let mut risks_list = column![].spacing(6);
                for risk in &diff.risks {
                    let r_title = if risk.title.is_empty() {
                        "Unknown"
                    } else {
                        &risk.title
                    };

                    let r_desc = risk
                        .description
                        .as_deref()
                        .filter(|s| !s.is_empty())
                        .unwrap_or("No risk description available");

                    risks_list = risks_list.push(
                        column![
                            text(r_title).size(14).font(bold_font()).color(ERROR_RED),
                            Element::from(text(r_desc).size(12).color(TEXT_DIM))
                        ]
                        .spacing(1),
                    );
                }

                let risks_block = column![
                    text("RISK MODIFIERS")
                        .size(11)
                        .font(bold_font())
                        .color(ERROR_RED),
                    risks_list,
                ]
                .spacing(4);

                difficulties_content = difficulties_content.push(risks_block);
            }
        }
    }

    container(row![
        container(Space::new().width(Length::Fixed(3.0)))
            .height(Length::Fill)
            .style(move |_theme| container::Style {
                background: Some(SOFT_CYAN.into()),
                ..Default::default()
            }),
        row![
            faction_icon(faction),
            column![
                text(format!("MISSION {}", index + 1))
                    .size(12)
                    .font(bold_font())
                    .color(SOFT_CYAN),
                text(mtype).size(18).font(bold_font()).color(Color::WHITE),
                Space::new().height(Length::Fixed(10.0)),
                difficulties_content,
            ]
            .width(Length::Fill)
        ]
        .padding(12)
        .spacing(12)
        .width(Length::Fill)
    ])
    .width(Length::Fill)
    .style(move |_theme| container::Style {
        background: Some(CARD_BG.into()),
        border: Border {
            color: Color {
                a: 0.1,
                ..Color::WHITE
            },
            width: 1.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    })
    .into()
}

pub fn render_archimedea(app: &VoidFissuresApp) -> Element<'_, Message> {
    match &app.fissures {
        DataState::Loading => container(text("ANALYZING WORLDSTATE...").size(18).color(SOFT_CYAN))
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
        DataState::Loaded(data) => {
            let root = &data.archimedea;
            if root.elite_deep.is_none() && root.elite_temporal.is_none() {
                return container(
                    text("NO ACTIVE ELITE ARCHIMEDEA CHALLENGES FOUND")
                        .size(16)
                        .color(TEXT_DIM),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into();
            }

            let mut available_modes = Vec::new();
            if root.elite_deep.is_some() {
                available_modes.push((1, "ELITE DEEP ARCHIMEDEA", &root.elite_deep));
            }
            if root.elite_temporal.is_some() {
                available_modes.push((3, "ELITE TEMPORAL ARCHIMEDEA", &root.elite_temporal));
            }

            let active_mode_info = available_modes
                .iter()
                .find(|(idx, _, _)| *idx == app.selected_archimedea_tab)
                .or_else(|| available_modes.first());

            if let Some(&(_idx, _label, archimedea_opt)) = active_mode_info
                && let Some(archimedea) = archimedea_opt
            {
                let sub_tabs_row = row(available_modes.iter().map(|&(idx, label, _)| {
                    let active = active_mode_info
                        .map(|(a_idx, _, _)| *a_idx == idx)
                        .unwrap_or(false);

                    button(
                        text(label)
                            .size(13)
                            .font(bold_font())
                            .align_x(Alignment::Center),
                    )
                    .padding([6, 16])
                    .on_press(Message::SwitchArchimedeaTab(idx))
                    .style(move |_theme, _status| {
                        let base_bg = if active {
                            Color {
                                a: 0.2,
                                ..SOFT_GOLD
                            }
                        } else {
                            Color {
                                a: 0.05,
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
                                radius: 4.0.into(),
                            },
                            ..Default::default()
                        }
                    })
                    .into()
                }))
                .spacing(10);

                let time_remaining = format!(
                    "EXPIRY: {}",
                    archimedea
                        .expiry
                        .format("%b %d, %Y %H:%M UTC")
                        .to_string()
                        .to_uppercase()
                );
                let time_text = text(time_remaining)
                    .size(14)
                    .font(bold_font())
                    .color(SOFT_CYAN);

                let top_bar = row![sub_tabs_row, Space::new().width(Length::Fill), time_text]
                    .align_y(Alignment::Center);

                let mut mission_cards = column![].spacing(12).width(Length::Fill);
                for (i, m) in archimedea.missions.iter().enumerate() {
                    mission_cards = mission_cards.push(render_archimedea_mission_card(i, m));
                }

                let mut variables_elements = Vec::new();
                if archimedea.variables.is_empty() {
                    variables_elements.push(
                        column![
                            text("Unknown").size(15).font(bold_font()).color(SOFT_GOLD),
                            text("No research modifiers details available")
                                .size(13)
                                .color(TEXT_DIM)
                        ]
                        .spacing(2)
                        .width(Length::Fixed(280.0))
                        .into(),
                    );
                } else {
                    for var in &archimedea.variables {
                        let v_title = if var.title.is_empty() {
                            "Unknown"
                        } else {
                            &var.title
                        };
                        let v_desc = var
                            .description
                            .as_deref()
                            .filter(|s| !s.is_empty())
                            .unwrap_or("No details available");

                        variables_elements.push(
                            column![
                                text(v_title).size(15).font(bold_font()).color(SOFT_GOLD),
                                Element::from(text(v_desc).size(13).color(TEXT_DIM))
                            ]
                            .spacing(2)
                            .width(Length::Fixed(280.0))
                            .into(),
                        );
                    }
                }

                let variables_card = container(column![
                    text("RESEARCH MODIFIERS")
                        .size(16)
                        .font(bold_font())
                        .color(SOFT_GOLD),
                    Space::new().height(Length::Fixed(12.0)),
                    row(variables_elements)
                        .spacing(20)
                        .wrap()
                        .vertical_spacing(15)
                ])
                .padding(15)
                .width(Length::Fill)
                .style(move |_theme| container::Style {
                    background: Some(CARD_BG.into()),
                    border: Border {
                        color: Color {
                            a: 0.1,
                            ..SOFT_GOLD
                        },
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                });

                let main_col = column![
                    top_bar,
                    Space::new().height(Length::Fixed(15.0)),
                    mission_cards,
                    Space::new().height(Length::Fixed(20.0)),
                    variables_card
                ]
                .width(Length::Fill)
                .spacing(10);

                return container(scrollable(
                    container(main_col)
                        .padding(padding::right(20))
                        .width(Length::Fill),
                ))
                .padding(padding::bottom(20))
                .into();
            }

            container(
                text("NO ARCHIMEDEA CHALLENGES ACTIVE")
                    .size(16)
                    .color(TEXT_DIM),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
        }
    }
}
