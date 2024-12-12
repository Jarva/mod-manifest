use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModInfo {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub id: i64,
    pub game_id: i64,
    pub name: String,
    pub slug: String,
    pub links: Links,
    pub summary: String,
    pub status: i64,
    pub download_count: i64,
    pub is_featured: bool,
    pub primary_category_id: i64,
    pub class_id: i64,
    pub authors: Vec<Author>,
    pub logo: Logo,
    pub main_file_id: i64,
    pub latest_files: Vec<LatestFile>,
    pub date_created: String,
    pub date_modified: String,
    pub date_released: String,
    pub allow_mod_distribution: bool,
    pub game_popularity_rank: i64,
    pub is_available: bool,
    pub has_comments_enabled: bool,
    pub thumbs_up_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    pub website_url: String,
    pub wiki_url: String,
    pub issues_url: String,
    pub source_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub avatar_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Logo {
    pub id: i64,
    pub mod_id: i64,
    pub title: String,
    pub description: String,
    pub thumbnail_url: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestFile {
    pub id: i64,
    pub game_id: i64,
    pub mod_id: i64,
    pub is_available: bool,
    pub display_name: String,
    pub file_name: String,
    pub release_type: i64,
    pub file_status: i64,
    pub file_date: String,
    pub file_length: i64,
    pub download_count: i64,
    pub download_url: String,
    pub game_versions: Vec<String>,
    pub alternate_file_id: i64,
    pub is_server_pack: bool,
    pub file_fingerprint: i64,
    pub file_size_on_disk: Option<i64>,
}
