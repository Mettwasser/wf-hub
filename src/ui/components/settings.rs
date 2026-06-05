use iced::{
    Alignment,
    Border,
    Color,
    Element,
    Length,
    widget::{
        button,
        column,
        row,
        slider,
        space,
        text,
    },
};

use crate::ui::{
    Message,
    VoidFissuresApp,
};
use super::theme::SOFT_CYAN;

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
