
use serde::{Serialize, de::DeserializeOwned};

pub async fn invoke<T: DeserializeOwned, A: Serialize>(cmd: &str, args: &A) -> T {
    let args = serde_wasm_bindgen::to_value(&args).unwrap();
    serde_wasm_bindgen::from_value(binding::invoke(cmd, args).await).unwrap()
}



mod binding {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
        pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
    }
}

