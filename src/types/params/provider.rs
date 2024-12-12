use std::str::FromStr;
use rocket::request::FromParam;
use rocket_okapi::JsonSchema;
use strum_macros::EnumString;

#[derive(Debug, EnumString, PartialEq, JsonSchema)]
#[schemars(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Provider {
  CurseForge,
  Modrinth
}

impl<'r> FromParam<'r> for Provider {
  type Error = &'r str;

  fn from_param(param: &'r str) -> Result<Self, Self::Error> {
    Provider::from_str(param).map_err(|_| "Invalid Value")
  }
}
