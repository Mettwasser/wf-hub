use std::{
    collections::HashSet,
    fs,
    path::Path,
};

use chrono::Utc;
use serde::{
    Deserialize,
    Serialize,
};
use worldstate_parser::{
    Fissure,
    FissureTier,
    MissionType,
};

#[derive(Debug, Clone)]
pub enum DataState<T> {
    Loading,
    Loaded(T),
    Error(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SteelPathFilter {
    Normal,
    SteelPath,
    Both,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FissureSubscription {
    pub tiers: HashSet<FissureTier>,
    pub mission_types: HashSet<MissionType>,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionState {
    pub normal: FissureSubscription,
    pub steel_path: FissureSubscription,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

impl Default for SubscriptionState {
    fn default() -> Self {
        Self {
            normal: FissureSubscription::default(),
            steel_path: FissureSubscription::default(),
            enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenWorldCycles {
    pub cetus: worldstate_parser::cycles::cetus::CetusCycle,
    pub cambion: worldstate_parser::cycles::cambion_drift::CambionDriftCycle,
    pub vallis: worldstate_parser::cycles::orb_vallis::OrbVallisCycle,
    #[serde(default)]
    pub cetus_bounties: Vec<worldstate_parser::SyndicateJob>,
    #[serde(default)]
    pub cambion_bounties: Vec<worldstate_parser::SyndicateJob>,
    #[serde(default)]
    pub vallis_bounties: Vec<worldstate_parser::SyndicateJob>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppData {
    pub fissures: Vec<Fissure>,
    pub archimedea: worldstate_parser::ArchimedeaRoot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastFetch {
    pub fissures: Vec<Fissure>,
    #[serde(default)]
    pub archimedea: Option<worldstate_parser::ArchimedeaRoot>,
    #[serde(default)]
    pub open_worlds: Option<OpenWorldCycles>,
    pub at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub active_filters: HashSet<FissureTier>,
    pub mission_filters: HashSet<MissionType>,
    pub steel_path_filter: SteelPathFilter,
    pub subscriptions: SubscriptionState,
    pub volume: f32,
    #[serde(default)]
    pub current_tab: usize,

    pub last_fetch: Option<LastFetch>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            active_filters: [
                FissureTier::Lith,
                FissureTier::Meso,
                FissureTier::Neo,
                FissureTier::Axi,
                FissureTier::Requiem,
                FissureTier::Omnia,
            ]
            .into_iter()
            .collect(),
            mission_filters: HashSet::new(),
            steel_path_filter: SteelPathFilter::Both,
            subscriptions: SubscriptionState::default(),
            volume: 1.0,
            current_tab: 0,
            last_fetch: None,
        }
    }
}

impl AppConfig {
    pub fn load() -> Self {
        let path = Path::new("config.json");
        if path.exists()
            && let Ok(content) = fs::read_to_string(path)
            && let Ok(config) = serde_json::from_str::<AppConfig>(&content)
        {
            return config;
        }

        Self::default()
    }

    pub fn save(&self) {
        let _ = serde_json::to_string_pretty(self).map(|json| fs::write("config.json", json));
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscription_state_defaults() {
        let state = SubscriptionState::default();
        assert!(state.enabled);

        let json = r#"{"normal":{"tiers":[],"mission_types":[]},"steel_path":{"tiers":[],"mission_types":[]}}"#;
        let deserialized: SubscriptionState = serde_json::from_str(json).unwrap();
        assert!(deserialized.enabled);
    }
}
