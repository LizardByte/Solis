#![doc = "Web server utilities for the application."]

// modules
mod routes;

// lib imports
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{rapidoc::*, swagger_ui::*};

/// Build the web server.
pub fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", routes::all_routes())
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new(
                        "General",
                        "../openapi.json",
                    )],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
}

/// Launch the web server.
#[rocket::main]
pub async fn launch() {
    rocket()
        .launch()
        .await
        .expect("Failed to launch web server");
}
