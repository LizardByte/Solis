#![doc(html_favicon_url = "../assets/icon.ico")]
#![doc(html_logo_url = "../assets/icon.png")]
#![doc = include_str!("../docs/README.md")]
#![deny(missing_docs)]

// modules
mod logging;
mod tray;
pub mod web;

// standard imports
use std::thread;

// global constants and variables
static GLOBAL_APP_NAME: &str = "Koko";
static GLOBAL_ICON_ICO_PATH: &str = "assets/icon.ico";
static GLOBAL_BASE_URL: &str = "http://localhost:8000"; // TODO: get this dynamically

/// Main entry point for the application.
/// Initializes logging, the web server, and tray icon.
pub fn main() {
    logging::init().expect("Failed to initialize logging");

    let web_handle = thread::spawn(|| {
        web::launch();
    });

    tray::launch();

    web_handle.join().expect("Web server thread panicked");
}
