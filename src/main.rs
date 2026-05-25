mod fissures;
mod notifications;
mod ui;

use std::sync::Arc;

use fissures::SubscriptionState;
use rodio::{
    DeviceSinkBuilder,
    Player,
};
use tokio::sync::watch;
use ui::VoidFissuresApp;

pub fn main() -> iced::Result {
    let file_appender = tracing_appender::rolling::never(".", "wf-hub.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_max_level(tracing::Level::INFO)
        .init();

    let (subscription_tx, subscription_rx) = watch::channel(SubscriptionState::default());

    let handle = DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let player = Arc::new(Player::connect_new(handle.mixer()));

    iced::application(
        move || {
            VoidFissuresApp::new(
                subscription_tx.clone(),
                subscription_rx.clone(),
                player.clone(),
            )
        },
        VoidFissuresApp::update,
        VoidFissuresApp::view,
    )
    .title("Void Fissures")
    .subscription(VoidFissuresApp::subscription)
    .theme(VoidFissuresApp::theme)
    .run()
}
