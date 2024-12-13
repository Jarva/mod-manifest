use crate::types::params::loader::Loader;
use anyhow::Result;

pub async fn get_mod_info(id: &str, loader: &Loader, version: Option<&str>) -> Result<String> {
    let query = if *loader == Loader::NeoForge {
        "?neoforge=only"
    } else {
        ""
    };

    let res = reqwest::get(format!("https://api.modrinth.com/updates/{}/forge_updates.json{}", id, query))
      .await.expect("Unable to retrieve forge_updates.json")
      .text()
      .await.expect("Unable to stringify forge_updates.json");

    Ok(res)
}
