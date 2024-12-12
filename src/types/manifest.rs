use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Manifest {
    pub homepage: String,
    pub promos: HashMap<String, String>,
    #[serde(flatten)]
    pub versions: IndexMap<String, IndexMap<String, String>>,
}
