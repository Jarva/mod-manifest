use crate::types::curseforge::files::File;
use anyhow::Result;
use crate::loaders::forge;

pub fn from_cf(url: String, files: Vec<File>) -> Result<String> {
    // Neoforge implementation is currently identical to Forge
    forge::from_cf(url, files)
}
