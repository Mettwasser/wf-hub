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
        slider,
        text,
        toggler,
    },
};

use super::theme::{
    CARD_BG,
    SOFT_CYAN,
    SOFT_GOLD,
    TEXT_DIM,
    bold_font,
};
use crate::ui::{
    Message,
    WarframeHubApp,
};

pub fn render_settings(app: &WarframeHubApp) -> Element<'_, Message> {
    // Notification Settings Card
    let notifications_card = container(column![
        row![
            text("NOTIFICATION SETTINGS")
                .size(14)
                .font(bold_font())
                .color(SOFT_GOLD),
            Space::new().width(Length::Fill),
            button(text("SEND TEST NOTIFICATION").size(10).font(bold_font()))
                .padding([6, 12])
                .on_press(Message::TestAlert)
                .style(move |_theme, _status| {
                    button::Style {
                        background: Some(Color::TRANSPARENT.into()),
                        text_color: SOFT_CYAN,
                        border: Border {
                            color: SOFT_CYAN,
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    }
                })
        ]
        .align_y(Alignment::Center),
        Space::new().height(Length::Fixed(16.0)),
        // Global Notification Master Switch
        column![
            row![
                text("GLOBAL NOTIFICATIONS")
                    .size(13)
                    .font(bold_font())
                    .color(Color::WHITE),
                Space::new().width(Length::Fill),
                toggler(app.subscriptions.global_enabled)
                    .label("")
                    .spacing(0)
                    .on_toggle(Message::ToggleGlobalNotifications),
            ]
            .align_y(Alignment::Center),
            text("Enable or disable all app notifications (Fissures, Cetus, etc.)")
                .size(11)
                .color(TEXT_DIM),
        ]
        .spacing(4),
        Space::new().height(Length::Fixed(16.0)),
        // Fissures Alert Switch
        column![
            row![
                text("VOID FISSURE ALERTS")
                    .size(13)
                    .font(bold_font())
                    .color(Color::WHITE),
                Space::new().width(Length::Fill),
                {
                    let mut t = toggler(app.subscriptions.fissures.enabled).label("").spacing(0);
                    if app.subscriptions.global_enabled {
                        t = t.on_toggle(Message::ToggleFissureNotifications);
                    }
                    t
                },
            ]
            .align_y(Alignment::Center),
            text("Get notified when a subscription-matching Void Fissure appears.")
                .size(11)
                .color(TEXT_DIM),
        ]
        .spacing(4),
        Space::new().height(Length::Fixed(16.0)),
        // Cetus Night Alert Switch
        column![
            row![
                text("CETUS NIGHT ALERTS")
                    .size(13)
                    .font(bold_font())
                    .color(Color::WHITE),
                Space::new().width(Length::Fill),
                {
                    let mut t = toggler(app.subscriptions.cetus_night_enabled)
                        .label("")
                        .spacing(0);
                    if app.subscriptions.global_enabled {
                        t = t.on_toggle(Message::ToggleCetusNotifications);
                    }
                    t
                },
            ]
            .align_y(Alignment::Center),
            text("Get notified when the Plains of Eidolon transition from Day to Night.")
                .size(11)
                .color(TEXT_DIM),
        ]
        .spacing(4),
    ])
    .padding(20)
    .style(|_| container::Style {
        background: Some(CARD_BG.into()),
        border: Border {
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.05),
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    });

    // Sound / Audio Settings Card
    let audio_card = container(column![
        text("AUDIO SETTINGS")
            .size(14)
            .font(bold_font())
            .color(SOFT_GOLD),
        Space::new().height(Length::Fixed(16.0)),
        row![
            column![
                text("ALERT VOLUME")
                    .size(13)
                    .font(bold_font())
                    .color(Color::WHITE),
                text("Adjust the playback volume of notification sounds.")
                    .size(11)
                    .color(TEXT_DIM),
            ]
            .spacing(4)
            .width(Length::Shrink),
            Space::new().width(Length::Fill),
            row![
                text(format!("{:.0}%", app.volume * 100.0))
                    .size(14)
                    .font(bold_font())
                    .color(Color::WHITE)
                    .width(Length::Fixed(45.0)),
                slider(
                    0.0..=100.0,
                    app.volume * 100.0,
                    |val| Message::ChangeVolume(val / 100.0)
                )
                .width(Length::Fixed(250.0)),
                Space::new().width(Length::Fixed(15.0)),
                button(text("TEST").size(11).font(bold_font()))
                    .padding([6, 12])
                    .on_press(Message::TestVolume)
                    .style(move |_theme, _status| {
                        button::Style {
                            background: Some(Color::TRANSPARENT.into()),
                            text_color: SOFT_GOLD,
                            border: Border {
                                color: SOFT_GOLD,
                                width: 1.0,
                                radius: 4.0.into(),
                            },
                            ..Default::default()
                        }
                    })
            ]
            .align_y(Alignment::Center)
            .width(Length::Shrink)
        ]
        .align_y(Alignment::Center)
    ])
    .padding(20)
    .style(|_| container::Style {
        background: Some(CARD_BG.into()),
        border: Border {
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.05),
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    });

    column![
        notifications_card,
        Space::new().height(Length::Fixed(20.0)),
        audio_card,
    ]
    .spacing(10)
    .into()
}
