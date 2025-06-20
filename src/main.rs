mod http;
mod loaders;
mod providers;
mod types;

#[macro_use]
extern crate rocket;

use crate::providers::{curseforge, modrinth};
use crate::types::params::{loader::Loader, provider::Provider};
use dotenvy::dotenv;
use rocket::http::{ContentType, Status};
use rocket_okapi::okapi::openapi3::{Info, OpenApi};
use rocket_okapi::openapi;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use std::env;
use rocket_sentry::RocketSentry;

#[get("/health")]
async fn healthcheck() -> String {
    String::from("OK")
}

#[openapi(tag = "Manifest")]
#[get("/manifest/<provider>/<id>/<loader>?<version>")]
async fn manifest(
    provider: Provider,
    id: &str,
    loader: Loader,
    version: Option<&str>,
) -> (Status, (ContentType, String)) {
    match provider {
        Provider::CurseForge => {
            if let Ok(json) = curseforge::get_mod_info(id, &loader, version).await {
                (Status::Ok, (ContentType::JSON, json))
            } else {
                (Status::InternalServerError, (ContentType::JSON, String::from("{}")))
            }
        },
        Provider::Modrinth => {
            if let Ok(json) = modrinth::get_mod_info(id, &loader, version).await {
                (Status::Ok, (ContentType::JSON, json))
            } else {
                (Status::InternalServerError, (ContentType::JSON, String::from("{}")))
            }
        },
        _ => {
            (
                Status::NotImplemented,
                (ContentType::Plain, String::from("Not Implemented")),
            )
        }
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    // env_logger::init();

    let settings = rocket_okapi::settings::OpenApiSettings::new();

    let mut spec: OpenApi = rocket_okapi::openapi_spec![manifest](&settings);
    spec.info = Info {
        title: "Mod Update Checker".to_string(),
        ..spec.info
    };
    let mut routes: Vec<rocket::Route> =
        rocket_okapi::openapi_routes![manifest, healthcheck](Some(spec), &settings);

    routes.extend(
        make_swagger_ui(&SwaggerUIConfig {
            url: "openapi.json".to_owned(),
            ..Default::default()
        })
        .into(),
    );

    rocket::build()
      .attach(RocketSentry::fairing())
      .mount("/", routes)
}
