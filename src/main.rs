pub mod fissure_producer;
mod fissures;
pub mod models;
mod notifications;
mod ui;

use crate::ui::VoidFissuresApp;

#[cfg(not(debug_assertions))]
fn init_tracing() {
    let file_appender = tracing_appender::rolling::never(".", "wf-hub.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_max_level(tracing::Level::INFO)
        .init();
}

#[cfg(debug_assertions)]
fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
}

fn main() -> iced::Result {
    init_tracing();

    iced::application(
        VoidFissuresApp::init,
        VoidFissuresApp::update,
        VoidFissuresApp::view,
    )
    .title("Warframe Hub")
    .subscription(VoidFissuresApp::tick_subscription)
    .theme(VoidFissuresApp::theme)
    .run()
}
