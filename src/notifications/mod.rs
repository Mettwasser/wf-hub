use crate::fissures::{SubscriptionState, fetch_fissures, mission_type_name};
use notify_rust::Notification;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::watch;
use tokio::time::{Duration, sleep};

pub async fn background_notification_task(
    client: Arc<reqwest::Client>,
    subscription_rx: watch::Receiver<SubscriptionState>,
) {
    let mut notified_ids: HashSet<String> = HashSet::new();

    loop {
        let subs = subscription_rx.borrow().clone();

        if (!subs.tiers.is_empty() || !subs.mission_types.is_empty())
            && let Ok(fissures) = fetch_fissures(client.clone()).await
        {
            for fissure in fissures {
                let matches_tier = subs.tiers.contains(&fissure.tier);
                let matches_mission = fissure
                    .node
                    .as_ref()
                    .map(|n| subs.mission_types.contains(&n.mission_type))
                    .unwrap_or(false);

                let is_match = match (subs.tiers.is_empty(), subs.mission_types.is_empty()) {
                    (false, false) => matches_tier && matches_mission,
                    (false, true) => matches_tier,
                    (true, false) => matches_mission,
                    (true, true) => false,
                };

                if is_match && !notified_ids.contains(&fissure.id) {
                    let node_name = fissure
                        .node
                        .as_ref()
                        .map(|n| n.name.clone())
                        .unwrap_or_else(|| "Unknown".to_string());
                    let mtype = fissure
                        .node
                        .as_ref()
                        .map(|n| mission_type_name(n.mission_type))
                        .unwrap_or_else(|| "Unknown".to_string());
                    let planet = fissure
                        .node
                        .as_ref()
                        .map(|n| n.planet.clone())
                        .unwrap_or_else(|| "Unknown".to_string());

                    let _ = Notification::new()
                        .summary("Fissure Alert")
                        .body(&format!(
                            "{:?} {mtype} at {node_name} ({planet})",
                            fissure.tier
                        ))
                        .show();

                    notified_ids.insert(fissure.id.clone());
                }
            }
        }

        sleep(Duration::from_mins(5)).await; // Poll every 5 minutes
    }
}
