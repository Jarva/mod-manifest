use crate::http::client::get_cf_client;
use crate::loaders::{forge, neoforge};
use crate::types::curseforge::files::{File, ModFiles};
use crate::types::curseforge::info::ModInfo;
use crate::types::params::loader::Loader;
use anyhow::Result;
use std::env;

pub async fn get_mod_info(id: &str, loader: &Loader, version: Option<&str>) -> Result<String> {
    let mod_loader_type = match loader {
        Loader::Forge => "1",
        Loader::NeoForge => "6"
    };

    let info_resp = get_cf_client()
        .get(format!("https://api.curseforge.com/v1/mods/{}", id))
        .header(
            "x-api-key",
            env::var("CF_API_KEY").expect("CF_API_KEY not set"),
        )
        .send()
        .await.expect("Unable to retrieve mod info")
        .json::<ModInfo>()
        .await.expect("Unable to decode mod info");

    let mut files: Vec<File> = Vec::new();
    loop {
        let mut files_builder = get_cf_client()
            .get(format!("https://api.curseforge.com/v1/mods/{}/files", id))
            .header(
                "x-api-key",
                env::var("CF_API_KEY").expect("CF_API_KEY not set"),
            )
            .query(&[
                ("index", files.len().to_string().as_str()),
                ("modLoaderType", mod_loader_type),
            ]);

        if let Some(version) = version {
            files_builder = files_builder.query(&[("gameVersion", version)]);
        }

        let mut file_resp = files_builder.send()
          .await.expect("Unable to retrieve mod files")
          .json::<ModFiles>()
          .await.expect("Unable to decode mod files");

        files.append(&mut file_resp.data);

        if files.len() == file_resp.pagination.total_count as usize {
            break;
        }
    }

    files.sort_by_key(|a| std::cmp::Reverse(a.file_date));

    match loader {
        Loader::Forge => forge::from_cf(info_resp.data.links.website_url, files),
        Loader::NeoForge => neoforge::from_cf(info_resp.data.links.website_url, files),
    }
}
