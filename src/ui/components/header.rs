use iced::{
    Alignment,
    Border,
    Color,
    Element,
    Length,
    widget::{
        button,
        column,
        container,
        row,
        text,
        Space,
    },
};

use crate::ui::{
    Message,
    VoidFissuresApp,
};
use super::theme::{
    SOFT_GOLD,
    TEXT_DIM,
    bold_font,
    refresh_button_style,
};

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
            row![title_text, Space::new().width(Length::Fill), action_buttons].align_y(Alignment::Center),
        )
        .padding(20)
        .width(Length::Fill)
        .into()
    }
}
