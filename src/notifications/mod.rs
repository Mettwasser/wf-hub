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
    let mut should_play_sound = false;

    loop {
        let subs = subscription_rx.borrow().clone();

        if let DataState::Loaded(ref fissures) = *fissures_rx.borrow() {
            for fissure in fissures {
                let sub = if fissure.is_steel_path {
                    &subs.steel_path
                } else {
                    &subs.normal
                };

                if sub.tiers.is_empty() && sub.mission_types.is_empty() {
                    continue;
                }

                let matches_tier = sub.tiers.is_empty() || sub.tiers.contains(&fissure.tier);
                let matches_mission = sub.mission_types.is_empty()
                    || fissure
                        .node
                        .as_ref()
                        .is_some_and(|n| sub.mission_types.contains(&n.mission_type));

                if matches_tier && matches_mission && !notified_ids.contains(&fissure.id) {
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
                            "{:?} {mtype} at {node_name} ({planet}){}",
                            fissure.tier,
                            if fissure.is_steel_path {
                                " - STEEL PATH"
                            } else {
                                ""
                            }
                        ))
                        .appname("Void Fissures")
                        .show();

                    should_play_sound = true;

                    notified_ids.insert(fissure.id.clone());
                }
            }

            if should_play_sound {
                let source = get_source();
                player.append(source);
                should_play_sound = false;
            }
        }

        let _ = fissures_rx.changed().await;
    }
}
