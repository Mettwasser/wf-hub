pub mod models;
mod notifications;
mod ui;
pub mod utils;
mod world_state;
pub mod world_state_producer;

use iced::window;

use crate::ui::WarframeHubApp;

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
        WarframeHubApp::init,
        WarframeHubApp::update,
        WarframeHubApp::view,
    )
    .title("Warframe Hub")
    .subscription(WarframeHubApp::tick_subscription)
    .theme(WarframeHubApp::theme)
    .window(window::Settings {
        #[cfg(target_os = "linux")]
        platform_specific: window::settings::PlatformSpecific {
            application_id: env!("CARGO_PKG_NAME").to_owned(),
            ..Default::default()
        },
        ..Default::default()
    })
    .run()
}
