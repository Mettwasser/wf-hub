use std::collections::HashSet;
use worldstate_parser::{FissureTier, MissionType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum DataState<T> {
    Loading,
    Loaded(T),
    Error(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SteelPathFilter {
    Normal,
    SteelPath,
    Both,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubscriptionState {
    pub tiers: HashSet<FissureTier>,
    pub mission_types: HashSet<MissionType>,
}

pub fn tier_to_int(tier: FissureTier) -> i32 {
    match tier {
        FissureTier::Lith => 1,
        FissureTier::Meso => 2,
        FissureTier::Neo => 3,
        FissureTier::Axi => 4,
        FissureTier::Requiem => 5,
        FissureTier::Omnia => 6,
    }
}

pub fn mission_type_name(mtype: MissionType) -> String {
    serde_json::to_value(mtype)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_uppercase()))
        .unwrap_or_else(|| format!("{:?}", mtype).to_uppercase())
}
