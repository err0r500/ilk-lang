use std::path::Path;

use serde_json::json;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn check(src: &str) -> String {
    match crate::compile(src, Path::new("playground.ilk")) {
        Ok(_) => json!({"ok": true}).to_string(),
        Err(errors) => json!({
            "ok": false,
            "errors": errors
        })
        .to_string(),
    }
}
