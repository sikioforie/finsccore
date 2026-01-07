#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::tauri;

static CSS: Asset = asset!("/assets/styles.css");
static TAURI_ICON: Asset = asset!("/assets/tauri.svg");
static DIOXUS_ICON: Asset = asset!("/assets/dioxus.png");


#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

pub fn App() -> Element {
    let mut name = use_signal(|| String::new());
    let mut greet_msg = use_signal(|| String::new());

    let greet = move |e: FormEvent| async move {
        e.prevent_default();
        if name.read().is_empty() {
            return;
        }

        let name = name.read();
        let args = GreetArgs { name: &*name };
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        let new_msg: String = tauri::invoke("greet", &args).await;
        greet_msg.set(new_msg);
    };

    rsx! {
        link { rel: "stylesheet", href: CSS }
        main {
            class: "container",
            h1 { "Welcome to Tauri + Dioxus" }

            div {
                class: "row",
                a {
                    href: "https://tauri.app",
                    target: "_blank",
                    img {
                        src: TAURI_ICON,
                        class: "logo tauri",
                         alt: "Tauri logo"
                    }
                }
                a {
                    href: "https://dioxuslabs.com/",
                    target: "_blank",
                    img {
                        src: DIOXUS_ICON,
                        class: "logo dioxus",
                        alt: "Dioxus logo"
                    }
                }
            }
            p { "Click on the Tauri and Dioxus logos to learn more." }

            form {
                class: "row",
                onsubmit: greet,
                input {
                    id: "greet-input",
                    placeholder: "Enter a name...",
                    value: "{name}",
                    oninput: move |event| name.set(event.value())
                }
                button { r#type: "submit", "Greet" }
            }
            p { "{greet_msg}" }
        }
    }
}
