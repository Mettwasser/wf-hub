use iced::{
    Alignment,
    Border,
    Color,
    Element,
    Length,
    widget::{
        Space,
        Stack,
        column,
        container,
        image,
        row,
        scrollable,
        text,
    },
};
use worldstate_parser::cycles::{
    Cycle,
    cambion_drift::CambionDriftState,
    cetus::CetusState,
    orb_vallis::OrbVallisState,
};

use super::theme::{
    BG_DARK,
    ERROR_RED,
    SOFT_CYAN,
    SOFT_GOLD,
    TEXT_DIM,
    bold_font,
};
use crate::{
    models::DataState,
    ui::{
        Message,
        VoidFissuresApp,
        images::{
            get_cambiondrift_image,
            get_orbvallis_image,
            get_poe_image,
        },
    },
};

struct WorldCardInfo {
    world_name: &'static str,
    image_handle: iced::widget::image::Handle,
    phase_name: String,
    phase_color: Color,
    time_left: String,
    progress: f32,
}

fn calculate_progress<S>(cycle: &Cycle<S>) -> f32 {
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
        ]
        .align_y(Alignment::End),
        Space::new().height(Length::Fixed(10.0)),
        progress_bar,
    ]
    .padding(24)
    .height(Length::Fill);

    let base_image = container(
        image(info.image_handle)
            .width(Length::Fill)
            .height(Length::Fill)
            .content_fit(iced::ContentFit::Cover),
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

pub fn render_open_worlds(app: &VoidFissuresApp) -> Element<'_, Message> {
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
                world_name: "PLAINS OF EIDOLON",
                image_handle: get_poe_image(),
                phase_name: cetus_phase_name.to_string(),
                phase_color: cetus_phase_color,
                time_left: cycles.cetus.time_left(),
                progress: calculate_progress(&cycles.cetus),
            };

            let vallis_phase_name = match cycles.vallis.state {
                OrbVallisState::Warm => "WARM",
                OrbVallisState::Cold => "COLD",
            };
            let vallis_phase_color = match cycles.vallis.state {
                OrbVallisState::Warm => SOFT_GOLD,
                OrbVallisState::Cold => SOFT_CYAN,
            };
            let vallis_info = WorldCardInfo {
                world_name: "ORB VALLIS",
                image_handle: get_orbvallis_image(),
                phase_name: vallis_phase_name.to_string(),
                phase_color: vallis_phase_color,
                time_left: cycles.vallis.time_left(),
                progress: calculate_progress(&cycles.vallis),
            };

            let cambion_phase_name = match cycles.cambion.state {
                CambionDriftState::Fass => "FASS",
                CambionDriftState::Vome => "VOME",
            };
            let cambion_phase_color = match cycles.cambion.state {
                CambionDriftState::Fass => Color::from_rgb(0.9, 0.4, 0.2),
                CambionDriftState::Vome => Color::from_rgb(0.3, 0.6, 0.9),
            };
            let cambion_info = WorldCardInfo {
                world_name: "CAMBION DRIFT",
                image_handle: get_cambiondrift_image(),
                phase_name: cambion_phase_name.to_string(),
                phase_color: cambion_phase_color,
                time_left: cycles.cambion.time_left(),
                progress: calculate_progress(&cycles.cambion),
            };

            scrollable(
                column![
                    render_card(cetus_info),
                    render_card(vallis_info),
                    render_card(cambion_info),
                ]
                .spacing(20)
                .padding(10),
            )
            .height(Length::Fill)
            .into()
        }
    }
}
