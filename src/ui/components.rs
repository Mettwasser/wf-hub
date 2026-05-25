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
    widget::{
        Space,
        button,
        column,
        container,
        row,
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
    fissures::{
        SteelPathFilter,
        mission_type_name,
    },
    ui::images::IMAGE_DIR,
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
                text("STEEL PATH")
                    .size(10)
                    .font(bold_font())
                    .color(ERROR_RED),
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
            .width(Length::Fixed(250.0)),
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
        FissureTier::Lith => Color::from_rgb(0.4, 0.6, 0.8),
        FissureTier::Meso => Color::from_rgb(0.7, 0.7, 0.3),
        FissureTier::Neo => Color::from_rgb(0.8, 0.5, 0.2),
        FissureTier::Axi => Color::from_rgb(0.7, 0.3, 0.3),
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
