use iced::{
    Alignment, Border, Color, Element, Length,
    widget::{column, container, row, scrollable, text, Space, Stack, image, button},
};
use worldstate_parser::cycles::{
    cetus::CetusState,
    orb_vallis::OrbVallisState,
    cambion_drift::CambionDriftState,
};

use super::theme::{
    bold_font, BG_DARK, CARD_BG, SOFT_GOLD, SOFT_CYAN, TEXT_DIM, ERROR_RED,
};
use crate::{
    models::DataState,
    ui::{
        Message,
        WarframeHubApp,
        OpenWorldId,
        images::{get_poe_image, get_orbvallis_image, get_cambiondrift_image},
    },
};

struct WorldCardInfo {
    world_id: OpenWorldId,
    world_name: &'static str,
    image_handle: iced::widget::image::Handle,
    phase_name: String,
    phase_color: Color,
    time_left: String,
    progress: f32,
    expanded: bool,
}

fn calculate_progress<S>(cycle: &worldstate_parser::cycles::Cycle<S>) -> f32 {
    let now = chrono::Utc::now();
    let total = (cycle.expiry - cycle.activation).num_seconds();
    if total <= 0 {
        return 0.0;
    }
    let remaining = (cycle.expiry - now).num_seconds().max(0);
    (remaining as f32 / total as f32).clamp(0.0, 1.0)
}

fn render_card<'a>(info: WorldCardInfo) -> Element<'a, Message> {
    let active_val = ((info.progress * 1000.0) as u16).max(1);
    let remaining_val = (((1.0 - info.progress) * 1000.0) as u16).max(1);

    let active_portion = container(Space::new())
        .width(Length::FillPortion(active_val))
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(info.phase_color.into()),
            border: Border::default(),
            ..Default::default()
        });

    let remaining_portion = container(Space::new())
        .width(Length::FillPortion(remaining_val))
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.1).into()),
            border: Border::default(),
            ..Default::default()
        });

    let progress_bar = row![active_portion, remaining_portion]
        .width(Length::Fill)
        .height(Length::Fixed(4.0));

    let card_content = column![
        row![
            text(info.world_name)
                .size(22)
                .font(bold_font())
                .color(Color::WHITE),
            Space::new().width(Length::Fill),
            container(
                text(info.phase_name)
                    .size(13)
                    .font(bold_font())
                    .color(Color::WHITE)
            )
            .padding([6, 12])
            .style(move |_| container::Style {
                background: Some(Color { a: 0.12, ..info.phase_color }.into()),
                border: Border {
                    color: Color { a: 0.25, ..info.phase_color },
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..Default::default()
            })
        ]
        .align_y(Alignment::Center),

        Space::new().height(Length::Fill),

        row![
            text("TIME REMAINING:")
                .size(11)
                .font(bold_font())
                .color(TEXT_DIM),
            Space::new().width(Length::Fixed(8.0)),
            text(info.time_left)
                .size(18)
                .font(bold_font())
                .color(Color::WHITE),
            Space::new().width(Length::Fill),
            button(
                text(if info.expanded { "▲ HIDE BOUNTIES" } else { "▼ SHOW BOUNTIES" })
                    .size(11)
                    .font(bold_font())
                    .color(SOFT_GOLD)
            )
            .padding([6, 12])
            .on_press(Message::ToggleBounties(info.world_id))
            .style(|_, _| button::Style {
                background: Some(Color::TRANSPARENT.into()),
                border: Border {
                    color: Color { a: 0.3, ..SOFT_GOLD },
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            })
        ]
        .align_y(Alignment::Center),

        Space::new().height(Length::Fixed(10.0)),

        progress_bar,
    ]
    .padding(24)
    .height(Length::Fill);

    let base_image = container(
        image(info.image_handle)
            .width(Length::Fill)
            .height(Length::Fill)
            .content_fit(iced::ContentFit::Cover)
    )
    .width(Length::Fill)
    .height(Length::Fixed(180.0))
    .padding(4)
    .style(|_| container::Style {
        background: Some(Color::BLACK.into()),
        border: Border {
            radius: 12.0.into(),
            ..Default::default()
        },
        ..Default::default()
    });

    let overlay = container(card_content)
        .width(Length::Fill)
        .height(Length::Fixed(180.0))
        .style(|_| container::Style {
            background: Some(Color::from_rgba(0.08, 0.08, 0.1, 0.75).into()),
            border: Border {
                color: BG_DARK,
                width: 4.0,
                radius: 12.0.into(),
            },
            ..Default::default()
        });

    Stack::with_children([base_image.into(), overlay.into()]).into()
}

fn rarity_color(rarity: &str) -> Color {
    match rarity.to_lowercase().as_str() {
        "rare" => Color::from_rgb(0.9, 0.75, 0.35),      // Warframe Gold
        "uncommon" => Color::from_rgb(0.75, 0.75, 0.8),  // Warframe Silver
        _ => Color::from_rgb(0.65, 0.45, 0.3),           // Warframe Bronze
    }
}

fn render_bounties_list<'a>(
    card_element: Element<'a, Message>,
    expanded: bool,
    bounties: &'a [worldstate_parser::SyndicateJob],
) -> Element<'a, Message> {
    let mut card_and_bounties = column![card_element];

    if expanded {
        let mut bounties_col = column![
            Space::new().height(Length::Fixed(6.0))
        ]
        .spacing(8)
        .padding(iced::padding::left(10));
        if bounties.is_empty() {
            bounties_col = bounties_col.push(
                container(
                    text("NO BOUNTIES AVAILABLE")
                        .size(13)
                        .font(bold_font())
                        .color(TEXT_DIM)
                )
                .padding(15)
                .width(Length::Fill)
                .style(|_| container::Style {
                    background: Some(CARD_BG.into()),
                    border: Border {
                        color: Color::from_rgba(1.0, 1.0, 1.0, 0.05),
                        width: 1.0,
                        radius: 6.0.into(),
                    },
                    ..Default::default()
                })
            );
        } else {
            for bounty in bounties {
                let title = bounty.job_type.clone().unwrap_or_else(|| "Bounty".to_string()).to_uppercase();
                let level_str = format!("LVL {} - {}", bounty.min_enemy_level, bounty.max_enemy_level);
                let standing: u64 = bounty.xp_amounts.iter().sum();
                let standing_str = format!("★ {} STANDING", standing);
                
                let rewards_chips = row(bounty.rewards.iter().map(|r| {
                    container(
                        text(r.item_name.clone())
                            .size(9)
                            .font(bold_font())
                            .color(rarity_color(&r.rarity))
                    )
                    .padding([3, 6])
                    .style(|_| container::Style {
                        background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.25).into()),
                        border: Border {
                            color: Color::from_rgba(1.0, 1.0, 1.0, 0.05),
                            width: 1.0,
                            radius: 3.0.into(),
                        },
                        ..Default::default()
                    })
                    .into()
                }).collect::<Vec<_>>())
                .spacing(6)
                .wrap()
                .vertical_spacing(6);

                let bounty_card = container(
                    column![
                        row![
                            text(title)
                                .size(13)
                                .font(bold_font())
                                .color(SOFT_GOLD),
                            Space::new().width(Length::Fill),
                            text(level_str)
                                .size(11)
                                .font(bold_font())
                                .color(TEXT_DIM),
                            Space::new().width(Length::Fixed(15.0)),
                            text(standing_str)
                                .size(11)
                                .font(bold_font())
                                .color(SOFT_CYAN),
                        ]
                        .align_y(Alignment::Center),
                        Space::new().height(Length::Fixed(8.0)),
                        rewards_chips,
                    ]
                )
                .padding(12)
                .width(Length::Fill)
                .style(|_| container::Style {
                    background: Some(CARD_BG.into()),
                    border: Border {
                        color: Color::from_rgba(1.0, 1.0, 1.0, 0.05),
                        width: 1.0,
                        radius: 6.0.into(),
                    },
                    ..Default::default()
                });
                bounties_col = bounties_col.push(bounty_card);
            }
        }
        card_and_bounties = card_and_bounties.push(bounties_col);
    }

    card_and_bounties.into()
}

pub fn render_open_worlds(app: &WarframeHubApp) -> Element<'_, Message> {
    match &app.world_state.open_worlds {
        DataState::Loading => container(
            text("ANALYZING OPEN WORLD CYCLES...")
                .size(18)
                .font(bold_font())
                .color(SOFT_GOLD),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .into(),

        DataState::Error(e) => container(
            column![
                text("ERROR LOADING WORLDSTATE")
                    .size(20)
                    .font(bold_font())
                    .color(ERROR_RED),
                Space::new().height(Length::Fixed(10.0)),
                text(e).size(14).color(TEXT_DIM),
            ]
            .align_x(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .into(),

        DataState::Loaded(cycles) => {
            let cetus_phase_name = match cycles.cetus.state {
                CetusState::Day => "DAY",
                CetusState::Night => "NIGHT",
            };
            let cetus_phase_color = match cycles.cetus.state {
                CetusState::Day => SOFT_GOLD,
                CetusState::Night => SOFT_CYAN,
            };
            let cetus_info = WorldCardInfo {
                world_id: OpenWorldId::Cetus,
                world_name: "PLAINS OF EIDOLON",
                image_handle: get_poe_image(),
                phase_name: cetus_phase_name.to_string(),
                phase_color: cetus_phase_color,
                time_left: cycles.cetus.time_left(),
                progress: calculate_progress(&cycles.cetus),
                expanded: app.cetus_expanded,
            };
            let cetus_card = render_card(cetus_info);
            let cetus_col = render_bounties_list(cetus_card, app.cetus_expanded, &cycles.cetus_bounties);

            let vallis_phase_name = match cycles.vallis.state {
                OrbVallisState::Warm => "WARM",
                OrbVallisState::Cold => "COLD",
            };
            let vallis_phase_color = match cycles.vallis.state {
                OrbVallisState::Warm => SOFT_GOLD,
                OrbVallisState::Cold => SOFT_CYAN,
            };
            let vallis_info = WorldCardInfo {
                world_id: OpenWorldId::Vallis,
                world_name: "ORB VALLIS",
                image_handle: get_orbvallis_image(),
                phase_name: vallis_phase_name.to_string(),
                phase_color: vallis_phase_color,
                time_left: cycles.vallis.time_left(),
                progress: calculate_progress(&cycles.vallis),
                expanded: app.vallis_expanded,
            };
            let vallis_card = render_card(vallis_info);
            let vallis_col = render_bounties_list(vallis_card, app.vallis_expanded, &cycles.vallis_bounties);

            let cambion_phase_name = match cycles.cambion.state {
                CambionDriftState::Fass => "FASS",
                CambionDriftState::Vome => "VOME",
            };
            let cambion_phase_color = match cycles.cambion.state {
                CambionDriftState::Fass => Color::from_rgb(0.9, 0.4, 0.2),
                CambionDriftState::Vome => Color::from_rgb(0.3, 0.6, 0.9),
            };
            let cambion_info = WorldCardInfo {
                world_id: OpenWorldId::Cambion,
                world_name: "CAMBION DRIFT",
                image_handle: get_cambiondrift_image(),
                phase_name: cambion_phase_name.to_string(),
                phase_color: cambion_phase_color,
                time_left: cycles.cambion.time_left(),
                progress: calculate_progress(&cycles.cambion),
                expanded: app.cambion_expanded,
            };
            let cambion_card = render_card(cambion_info);
            let cambion_col = render_bounties_list(cambion_card, app.cambion_expanded, &cycles.cambion_bounties);

            scrollable(
                column![
                    cetus_col,
                    vallis_col,
                    cambion_col,
                ]
                .spacing(20)
                .padding(10),
            )
            .height(Length::Fill)
            .into()
        }
    }
}
