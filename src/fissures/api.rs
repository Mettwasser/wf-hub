use std::path::Path;

use worldstate_parser::{
    Fissure,
    WorldState,
    default_context_provider::{
        DefaultContextProvider,
        PathContext,
    },
};

pub async fn fetch_world_state(
    client: &reqwest::Client,
) -> Result<
    (
        Vec<Fissure>,
        worldstate_parser::ArchimedeaRoot,
        crate::models::OpenWorldCycles,
    ),
    String,
> {
    let url = "https://api.warframe.com/cdn/worldState.php";
    let raw_json = client
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;

    let provider = DefaultContextProvider(
        PathContext {
            data_dir: Path::new("data/"),
            drops_dir: Path::new("drops/"),
            assets_dir: Path::new("assets/"),
        },
        client,
    );

    let worldstate = WorldState::from_str(&raw_json, provider)
        .await
        .map_err(|e| format!("Parser Error: {:?}", e))?;

    tracing::debug!(?worldstate.fissures);

    let open_worlds = crate::models::OpenWorldCycles {
        cetus: worldstate.cetus_cycle,
        cambion: worldstate.cambion_drift_cycle,
        vallis: worldstate.orb_vallis_cycle,
    };

    Ok((worldstate.fissures, worldstate.archimedea, open_worlds))
}

