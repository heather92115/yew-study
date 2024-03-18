/*

use std::collections::HashMap;
use std::fmt::format;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};


async fn load_lang(lang: &str) -> Result<HashMap<String, String>, Err> {

    let map:HashMap<String, String> = HashMap::new();

    let lang_url = format!("/assets/{}.json", lang);
    let client = reqwest::Client::new();
    let res = client.get(lang_url).send().await?;

    serde_json::from_str(&res.json())?;


    Err("implme")
}

 */
