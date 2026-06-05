use iced::{
    Border,
    Color,
    Font,
    Theme,
    font::Weight,
    widget::button,
};

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
