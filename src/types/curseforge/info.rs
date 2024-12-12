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
    pub wiki_url: Option<String>,
    pub issues_url: Option<String>,
    pub source_url: Option<String>,
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
