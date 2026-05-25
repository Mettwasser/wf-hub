use std::{
    sync::Arc,
    time::Duration,
};

use tokio::sync::{
    Notify,
    watch,
};
use worldstate_parser::{
    Fissure,
    default_data_fetcher::CacheStrategy,
};

use crate::fissures::{
    DataState,
    fetch_fissures,
};

pub async fn fissure_event_producer(
    tx: watch::Sender<DataState<Vec<Fissure>>>,
    refresh_signal: Arc<Notify>,
) {
    let client = reqwest::Client::new();

    worldstate_parser::default_data_fetcher::fetch_all(CacheStrategy::Duration(
        Duration::from_hours(72),
    ))
    .await
    .expect("Worldstate initialization failed");

    loop {
        let fissures = fetch_fissures(&client).await;

        let data_state = match fissures {
            Ok(fissures) => DataState::Loaded(fissures),
            Err(e) => DataState::Error(e.to_string()),
        };

        let _ = tx.send(data_state);

        tokio::select! {
            _ = tokio::time::sleep(Duration::from_mins(5)) => {
                tracing::info!("Slept 5min");
            }

            _ = refresh_signal.notified() => {
                tracing::info!("User triggered manual refresh");
            }
        }
    }
}
