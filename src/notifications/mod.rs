use std::{
    collections::HashSet,
    io::Cursor,
    sync::Arc,
};

use notify_rust::Notification;
use rodio::Decoder;
use tokio::{
    self,
    sync::watch,
};
use worldstate_parser::Fissure;

use crate::models::{
    DataState,
    SubscriptionState,
    mission_type_name,
};

const FILE_CONTENTS: &[u8] = include_bytes!("../../sounds/notification.mp3");

pub fn get_source() -> Decoder<Cursor<&'static [u8]>> {
    Decoder::new_mp3(Cursor::new(FILE_CONTENTS)).unwrap()
}

pub async fn background_notification_task(
    subscription_rx: watch::Receiver<SubscriptionState>,
    player: Arc<rodio::Player>,
    mut fissures_rx: watch::Receiver<DataState<Vec<Fissure>>>,
) {
    let mut notified_ids: HashSet<String> = HashSet::new();
    let mut should_notify = false;

    loop {
        let subs = subscription_rx.borrow().clone();

        if (!subs.tiers.is_empty() || !subs.mission_types.is_empty())
            && let DataState::Loaded(ref fissures) = *fissures_rx.borrow()
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
                        .appname("Void Fissures")
                        .show();

                    should_notify = true;

                    notified_ids.insert(fissure.id.clone());
                }
            }

            if should_notify {
                let source = get_source();
                player.append(source);
                should_notify = false;
            }
        }

        let _ = fissures_rx.changed().await;
    }
}
