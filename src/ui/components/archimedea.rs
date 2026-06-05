use iced::{
    Alignment,
    Border,
    Color,
    Element,
    Length,
    widget::{
        Space,
        button,
        column,
        container,
        row,
        scrollable,
        text,
    },
};
use worldstate_parser::ArchimedeaMission;

use super::{
    fissures::faction_icon,
    theme::{
        CARD_BG,
        ERROR_RED,
        SOFT_CYAN,
        SOFT_GOLD,
        TEXT_DIM,
        bold_font,
    },
};
use crate::{
    models::{
        DataState,
        mission_type_name,
    },
    ui::{
        Message,
        VoidFissuresApp,
    },
};

fn render_archimedea_mission_card<'a>(
    index: usize,
    mission: &'a ArchimedeaMission,
) -> Element<'a, Message> {
    let faction = mission.faction;
    let mtype = mission_type_name(mission.mission_type);

    let mut difficulties_content = column![].spacing(10);

    if mission.difficulties.is_empty() {
        // Fallback for empty difficulties list
        let deviation_block = column![
            text("DEVIATION")
                .size(11)
                .font(bold_font())
                .color(SOFT_CYAN),
            text("Unknown")
                .size(15)
                .font(bold_font())
                .color(Color::WHITE),
            text("No deviation details available")
                .size(13)
                .color(TEXT_DIM),
        ]
        .spacing(2)
        .width(Length::FillPortion(1));

        let risks_block = column![
            text("RISK MODIFIERS")
                .size(11)
                .font(bold_font())
                .color(ERROR_RED),
            column![
                text("Unknown").size(14).font(bold_font()).color(ERROR_RED),
                text("No risk details available").size(12).color(TEXT_DIM)
            ]
            .spacing(1),
        ]
        .spacing(4)
        .width(Length::FillPortion(1));

        let diff_row = row![
            risks_block,
            Space::new().width(Length::Fixed(20.0)),
            deviation_block
        ]
        .width(Length::Fill);

        difficulties_content = difficulties_content.push(diff_row);
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
                    .color(SOFT_CYAN),
                text(dev_title)
                    .size(15)
                    .font(bold_font())
                    .color(Color::WHITE),
                text(dev_desc).size(13).color(TEXT_DIM),
            ]
            .spacing(2)
            .width(Length::FillPortion(1));

            let risks_block = if diff.risks.is_empty() {
                column![
                    text("RISK MODIFIERS")
                        .size(11)
                        .font(bold_font())
                        .color(ERROR_RED),
                    column![
                        text("Unknown").size(14).font(bold_font()).color(ERROR_RED),
                        text("No risk details available").size(12).color(TEXT_DIM)
                    ]
                    .spacing(1),
                ]
                .spacing(4)
                .width(Length::FillPortion(1))
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
                            Element::from(text(r_desc).size(12).color(TEXT_DIM)),
                        ]
                        .spacing(1),
                    );
                }

                column![
                    text("RISK MODIFIERS")
                        .size(11)
                        .font(bold_font())
                        .color(ERROR_RED),
                    risks_list,
                ]
                .spacing(4)
                .width(Length::FillPortion(1))
            };

            let diff_row = row![
                risks_block,
                Space::new().width(Length::Fixed(20.0)),
                deviation_block
            ]
            .width(Length::Fill);

            difficulties_content = difficulties_content.push(diff_row);
        }
    }

    container(row![
        container(Space::new().width(Length::Fixed(3.0)))
            .height(Length::Fill)
            .style(move |_theme| container::Style {
                background: Some(SOFT_GOLD.into()),
                ..Default::default()
            }),
        row![
            faction_icon(faction),
            column![
                text(format!("MISSION {}", index + 1))
                    .size(12)
                    .font(bold_font())
                    .color(SOFT_GOLD),
                text(mtype).size(18).font(bold_font()).color(Color::WHITE),
                Space::new().height(Length::Fixed(10.0)),
                difficulties_content,
            ]
            .width(Length::Fill)
        ]
        .padding([18, 12])
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
    match &app.world_state.archimedea {
        DataState::Loading => container(text("ANALYZING WORLDSTATE...").size(18).color(SOFT_GOLD))
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
        DataState::Loaded(root) => {
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

                let date_str = archimedea
                    .expiry
                    .with_timezone(&chrono::Local)
                    .format("%a, %b %-d @ %H:%M")
                    .to_string()
                    .to_uppercase();

                let time_row = row![
                    text("EXPIRY: ").size(14).font(bold_font()).color(SOFT_GOLD),
                    text(date_str).size(14).font(bold_font()).color(SOFT_CYAN),
                ];

                let top_bar = row![sub_tabs_row, Space::new().width(Length::Fill), time_row]
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
                                Element::from(text(v_desc).size(13).color(TEXT_DIM)),
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
                    variables_card,
                    Space::new().height(Length::Fixed(20.0)),
                    mission_cards
                ]
                .width(Length::Fill)
                .spacing(10);

                return container(scrollable(
                    container(main_col)
                        .padding(iced::padding::right(20))
                        .width(Length::Fill),
                ))
                .padding(iced::padding::bottom(20))
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
