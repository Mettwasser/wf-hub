use std::{
    path::Path,
    sync::Arc,
};

use worldstate_parser::{
    Fissure,
    WorldState,
    default_context_provider::{
        DefaultContextProvider,
        PathContext,
    },
};

pub async fn fetch_fissures(client: Arc<reqwest::Client>) -> Result<Vec<Fissure>, String> {
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
        &client,
    );

    let worldstate = WorldState::from_str(&raw_json, provider)
        .await
        .map_err(|e| format!("Parser Error: {:?}", e))?;

    tracing::info!(?worldstate.fissures);

    Ok(worldstate.fissures)
}
