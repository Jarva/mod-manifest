use rocket::request::FromParam;
use rocket_okapi::JsonSchema;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, EnumString, PartialEq, JsonSchema, Hash, Eq)]
#[schemars(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Loader {
    // Fabric,
    NeoForge,
    Forge,
}

impl<'r> FromParam<'r> for Loader {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Loader::from_str(param).map_err(|_| "Invalid Value")
    }
}
