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
        text,
    },
};

use super::theme::{
    SOFT_GOLD,
    TEXT_DIM,
    bold_font,
};
use crate::ui::{
    Message,
    WarframeHubApp,
};

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
    .padding(iced::padding::right(12))
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

pub fn render_sidebar(app: &WarframeHubApp) -> Element<'_, Message> {
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
    .padding(iced::padding::bottom(20));

    let menu = column![
        sidebar_button("VOID FISSURES", app.current_tab == 0, Message::SwitchTab(0)),
        sidebar_button(
            "ELITE ARCHIMEDEA",
            app.current_tab == 1,
            Message::SwitchTab(1)
        ),
        sidebar_button("OPEN WORLDS", app.current_tab == 2, Message::SwitchTab(2)),
    ]
    .spacing(8);

    container(
        column![
            logo,
            menu,
            Space::new().height(Length::Fill),
            sidebar_button("SETTINGS", app.current_tab == 3, Message::SwitchTab(3)),
        ]
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
