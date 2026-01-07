mod tauri;
mod ui;

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use ui::Route;

static CSS: Asset = asset!("/assets/styles.css");

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(|| {
        rsx! {
            link { rel: "stylesheet", href: CSS }
            Router::<Route> {}
        }
    });
}
