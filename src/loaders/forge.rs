use crate::types::curseforge::enums::FileReleaseType;
use crate::types::curseforge::files::File;
use crate::types::manifest::Manifest;
use anyhow::Result;
use fancy_regex::Regex;
use indexmap::IndexMap;
use std::collections::HashMap;

pub fn from_cf(url: String, files: Vec<File>) -> Result<String> {
    let file_version_finder =
        Regex::new(r"(?:^|[^0-9])(?<version>\d+(?:\.\d+)*)(?=[^0-9]|$)").unwrap();

    let mut mcversions = IndexMap::new();
    let mut promos = HashMap::new();

    for file in files {
        if !file.is_available {
            continue;
        }
        if file.release_type != FileReleaseType::Release {
            continue;
        }

        let file_name = file.file_name.clone();
        let captures = file_version_finder.captures_iter(file_name.as_str());

        let mut file_version = None;
        let mut last_found = None;
        for capture in captures {
            if capture.is_ok() {
                let found = capture
                    .unwrap()
                    .name("version")
                    .unwrap()
                    .as_str()
                    .to_string();
                let did_find = file
                    .game_versions
                    .iter()
                    .any(|version| found.contains(version.as_str()));
                if !did_find {
                    file_version = Some(found);
                    break;
                }
                last_found = Some(found);
            }
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
                mcversions.insert(version.to_string(), IndexMap::new());
            }
            let mcversion = mcversions.get_mut(version).unwrap();
            mcversion.insert(
                file_version.clone().unwrap().to_string(),
                format!("{}/files/{}", url, file.id),
            );
            if !promos.contains_key(version) {
                promos.insert(
                    version.to_string(),
                    file_version.clone().unwrap().to_string(),
                );
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
