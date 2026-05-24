mod fissures;
mod ui;
mod notifications;

use ui::VoidFissuresApp;
use tokio::sync::watch;
use fissures::SubscriptionState;

pub fn main() -> iced::Result {
    let file_appender = tracing_appender::rolling::never(".", "wf-hub.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_max_level(tracing::Level::INFO)
        .init();

    let (subscription_tx, subscription_rx) = watch::channel(SubscriptionState::default());

    iced::application(
        move || VoidFissuresApp::new(subscription_tx.clone(), subscription_rx.clone()),
        VoidFissuresApp::update,
        VoidFissuresApp::view,
    )
    .title("Void Fissures")
    .subscription(VoidFissuresApp::subscription)
    .theme(VoidFissuresApp::theme)
    .run()
}
