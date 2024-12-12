use std::collections::HashMap;
use std::env;
use anyhow::Result;
use crate::http::client::get_cf_client;
use crate::loaders::{forge, neoforge};
use crate::types::curseforge::files::{File, ModFiles};
use crate::types::curseforge::info::ModInfo;
use crate::types::params::loader::Loader;

pub async fn get_mod_info(id: &str, loader: &Loader, version: Option<&str>) -> Result<String> {
  let mod_loader_types = HashMap::from([
    (Loader::Forge, 1),
    (Loader::NeoForge, 6),
  ]);

  let info_resp = get_cf_client()?
    .get(format!("https://api.curseforge.com/v1/mods/{}", id))
    .header("x-api-key", env::var("CF_API_KEY").expect("CF_API_KEY not set"))
    .send().await?.json::<ModInfo>().await?;

  let mut files: Vec<File> = Vec::new();
  loop {
    let mod_loader_type = mod_loader_types.get(loader).expect("Unable to retrieve loader type").to_string();
    let mut files_builder = get_cf_client()?
      .get(format!("https://api.curseforge.com/v1/mods/{}/files", id))
      .header("x-api-key", env::var("CF_API_KEY").expect("CF_API_KEY not set"))
      .query(&[
        ("index", files.len().to_string().as_str()),
        ("modLoaderType", mod_loader_type.as_str())
      ]);

    if version.is_some() {
      files_builder = files_builder.query(&[("gameVersion", version.unwrap())]);
    }

    let mut file_resp = files_builder.send()
      .await?
      .json::<ModFiles>()
      .await?;

    files.append(&mut file_resp.data);

    if files.len().eq(&(file_resp.pagination.total_count as usize)) {
      break;
    }
  }

  files.sort_by(|a, b| a.file_date.cmp(&b.file_date));
  files.reverse();

  match loader {
    Loader::Forge => {
      forge::from_cf(info_resp.data.links.website_url, files)
    },
    Loader::NeoForge => {
      neoforge::from_cf(info_resp.data.links.website_url, files)
    }
  }
}
