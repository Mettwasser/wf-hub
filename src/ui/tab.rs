use iced::{
    Theme,
    widget::{
        Button,
        Row,
        button::{
            self,
            Status,
        },
        row,
    },
};

pub fn tabs<'a, Message: Clone + 'a>(
    tabs: impl IntoIterator<Item = Button<'a, Message>>,
    current_idx: usize,
    on_press: impl Fn(usize) -> Message,
    current_tab_style: impl Fn(&Theme, Status) -> button::Style + Clone + 'static,
) -> Row<'a, Message> {
    row(tabs.into_iter().enumerate().map(move |(c, btn)| {
        if c == current_idx {
            btn.on_press_maybe(None)
                .style(current_tab_style.clone())
                .into()
        } else {
            btn.on_press(on_press(c)).into()
        }
    }))
}
