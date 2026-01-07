#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::tauri;



#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    CalculateScore,

    #[route("/score/:id")]
    ScoreDetails{id:String},
}


#[component]
fn ScoreDetails(id: String) -> Element {
    rsx! { "User page for user with id: {id}" }
}


#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

pub fn CalculateScore() -> Element {
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
        div {
            class: "container",
            h1 { "Welcome to Tauri + Dioxus" }

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
