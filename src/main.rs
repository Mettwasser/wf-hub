pub mod fissure_producer;
mod fissures;
mod notifications;
mod ui;

use crate::ui::VoidFissuresApp;

fn main() -> iced::Result {
    let file_appender = tracing_appender::rolling::never(".", "wf-hub.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_max_level(tracing::Level::INFO)
        .init();

    iced::application(
        VoidFissuresApp::init,
        VoidFissuresApp::update,
        VoidFissuresApp::view,
    )
    .title("Void Fissures")
    .subscription(VoidFissuresApp::tick_subscription)
    .theme(VoidFissuresApp::theme)
    .run()
}
