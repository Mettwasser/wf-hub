pub mod fissure_producer;
mod fissures;
mod notifications;
mod ui;

use std::sync::Arc;

use fissures::SubscriptionState;
use rodio::{
    DeviceSinkBuilder,
    Player,
};
use tokio::{
    runtime::Runtime,
    sync::{
        Notify,
        watch,
    },
};
use worldstate_parser::Fissure;

use crate::{
    fissure_producer::fissure_event_producer,
    fissures::DataState,
    notifications::background_notification_task,
    ui::VoidFissuresApp,
};

fn main() -> iced::Result {
    let file_appender = tracing_appender::rolling::never(".", "wf-hub.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_max_level(tracing::Level::INFO)
        .init();

    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    let _rt_guard = rt.enter();

    let (subscription_tx, subscription_rx) = watch::channel(SubscriptionState::default());
    let (fissure_tx, fissure_rx) = watch::channel(DataState::<Vec<Fissure>>::Loading);
    let notify = Arc::new(Notify::new());

    let handle = DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let player = Arc::new(Player::connect_new(handle.mixer()));

    // Spawn background notification task
    tokio::spawn(background_notification_task(
        subscription_rx,
        player.clone(),
        fissure_rx.clone(),
    ));

    tokio::spawn(fissure_event_producer(fissure_tx, notify.clone()));

    iced::application(
        move || {
            VoidFissuresApp::new(
                subscription_tx.clone(),
                player.clone(),
                notify.clone(),
                fissure_rx.clone(),
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
