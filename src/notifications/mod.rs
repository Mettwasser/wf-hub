use std::{
    collections::HashSet,
    io::Cursor,
    sync::Arc,
};

use rodio::Decoder;
use tokio::{
    self,
    sync::watch,
};

use crate::{
    models::{
        DataState,
        SubscriptionState,
        mission_type_name,
    },
    utils::notification,
};
use worldstate_parser::cycles::cetus::CetusState;

const FILE_CONTENTS: &[u8] = include_bytes!("../../sounds/notification.mp3");

pub fn get_source() -> Decoder<Cursor<&'static [u8]>> {
    Decoder::new_mp3(Cursor::new(FILE_CONTENTS)).unwrap()
}

pub async fn background_notification_task(
    subscription_rx: watch::Receiver<SubscriptionState>,
    player: Arc<rodio::Player>,
    mut fissures_rx: watch::Receiver<DataState<Vec<worldstate_parser::Fissure>>>,
) {
    let mut notified_ids: HashSet<String> = HashSet::new();
    let mut should_play_sound = false;
    let mut first_fetch = true;

    loop {
        if let DataState::Loaded(ref fissures) = *fissures_rx.borrow() {
            let subs = subscription_rx.borrow();

            // for app startup; pollute notified ids so we don't send a notification on startup
            if first_fetch {
                notified_ids.extend(fissures.iter().map(|f| f.id.clone()));
            }

            for fissure in fissures {
                let sub = if fissure.is_steel_path {
                    &subs.fissures.steel_path
                } else {
                    &subs.fissures.normal
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
                    if subs.global_enabled && subs.fissures.enabled {
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

                        let steel_path_tag = if fissure.is_steel_path {
                            " - STEEL PATH"
                        } else {
                            ""
                        };

                        let _ = notification()
                            .summary("Fissure Alert")
                            .body(&format!(
                                "{:?} {mtype} at {node_name} ({planet}){steel_path_tag}",
                                fissure.tier,
                            ))
                            .show();

                        should_play_sound = true;
                    }

                    notified_ids.insert(fissure.id.clone());
                }
            }

            first_fetch = false;

            if should_play_sound {
                let source = get_source();
                player.append(source);
                should_play_sound = false;
            }

            let current_ids: HashSet<&String> = fissures.iter().map(|f| &f.id).collect();
            notified_ids.retain(|id| current_ids.contains(id));
        }

        tracing::info!(first_fetch, ?notified_ids, "Completed iteration");
        let _ = fissures_rx.changed().await;
    }
}

pub async fn cetus_notification_task(
    subscription_rx: watch::Receiver<SubscriptionState>,
    player: Arc<rodio::Player>,
    mut open_worlds_rx: watch::Receiver<DataState<crate::models::OpenWorldCycles>>,
) {
    let mut last_state = None;

    loop {
        if let DataState::Loaded(ref cycles) = *open_worlds_rx.borrow() {
            let current_state = cycles.cetus.state;

            if let Some(prev) = last_state && prev == CetusState::Day && current_state == CetusState::Night {
                let subs = subscription_rx.borrow();
                if subs.global_enabled && subs.cetus_night_enabled {
                    let _ = notification()
                        .summary("Cetus Alert")
                        .body("Night has fallen on the Plains of Eidolon!")
                        .show();

                    let source = get_source();
                    player.append(source);
                }
            }
            last_state = Some(current_state);
        }

        let _ = open_worlds_rx.changed().await;
    }
}
