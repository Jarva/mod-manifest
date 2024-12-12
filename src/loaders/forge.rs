use crate::types::curseforge::files::File;
use crate::types::manifest::Manifest;
use anyhow::Result;
use fancy_regex::Regex;
use indexmap::IndexMap;
use std::collections::HashMap;
use std::sync::LazyLock;
use crate::types::curseforge::enums::FileReleaseType;

static FILE_VERSION_FINDER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?:^|[^0-9])(?<version>\d+(?:\.\d+-?(?:beta|alpha)?)*)(?=[^0-9]|$)").unwrap());

pub fn from_cf(url: String, files: Vec<File>) -> Result<String> {
    let mut mcversions = IndexMap::new();
    let mut promos = HashMap::new();

    for file in files {
        if !file.is_available {
            continue;
        }

        let file_name = file.file_name.clone();
        let captures = FILE_VERSION_FINDER.captures_iter(file_name.as_str());

        let mut file_version = None;
        let mut last_found = None;
        for capture in captures.filter_map(Result::ok) {
            let found = capture.name("version").unwrap().as_str().to_string();
            let did_find = file.game_versions.iter().any(|version| found.contains(version));
            if !did_find {
                file_version = Some(found);
                break;
            }
            last_found = Some(found);
        }

        if file_version.is_none() {
            if last_found.is_some() {
                file_version = last_found;
            } else {
                continue;
            }
        }

        let versions = file.game_versions.iter().filter(|a| a.contains("."));
        for version in versions {
            if !mcversions.contains_key(version) {
                mcversions.insert(version.to_owned(), IndexMap::new());
            }
            let mcversion = mcversions.get_mut(version).unwrap();
            mcversion.insert(
                file_version.clone().unwrap(),
                format!("{}/files/{}", url, file.id),
            );
            let latest = version.to_owned() + "-latest";
            promos.entry(latest).or_insert_with(|| file_version.clone().unwrap());
            if file.release_type == FileReleaseType::Release {
                let recommended = version.to_owned() + "-recommended";
                promos.entry(recommended).or_insert_with(|| file_version.clone().unwrap());
            }
        }
    }

    let manifest = Manifest {
        homepage: url,
        versions: mcversions,
        promos,
    };

    serde_json::to_string(&manifest).map_err(anyhow::Error::new)
}
