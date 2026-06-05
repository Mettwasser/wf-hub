use std::{
    sync::Arc,
    time::Duration,
};

use tokio::sync::{
    Notify,
    watch,
};
use worldstate_parser::default_data_fetcher::CacheStrategy;

use crate::{
    fissures::fetch_world_state,
    models::{
        AppConfig,
        AppData,
        DataState,
    },
};

pub async fn fissure_event_producer(
    tx: watch::Sender<DataState<Box<AppData>>>,
    refresh_signal: Arc<Notify>,
) {
    let client = reqwest::Client::new();

    worldstate_parser::default_data_fetcher::fetch_all(CacheStrategy::Duration(
        Duration::from_hours(72),
    ))
    .await
    .expect("Worldstate initialization failed");

    let config = AppConfig::load();
    if let Some(last_fetch) = config.last_fetch {
        let elapsed = chrono::Utc::now() - last_fetch.at;
        let remaining = chrono::Duration::minutes(5) - elapsed;

        if remaining.num_seconds() > 0 {
            tracing::info!(
                "Respecting persistent fetch timer: sleeping for {}s",
                remaining.num_seconds()
            );
            tokio::select! {
                _ = tokio::time::sleep(Duration::from_secs(remaining.num_seconds() as u64)) => {
                    tracing::info!("Persistent sleep finished");
                }
                _ = refresh_signal.notified() => {
                    tracing::info!("User triggered manual refresh during persistent sleep");
                }
            }
        }
    }

    loop {
        let res = fetch_world_state(&client).await;

        let data_state = match res {
            Ok((fissures, archimedea)) => DataState::Loaded(Box::new(AppData {
                fissures,
                archimedea,
            })),
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
