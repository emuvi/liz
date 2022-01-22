use reqwest::header::HeaderMap;
use reqwest::header::HeaderName;

use std::collections::HashMap;

use crate::LizError;

pub fn get_text(url: &str, with_headers: Option<HashMap<String, String>>) -> Result<String, LizError> {
    let client = reqwest::blocking::Client::new();
    let builder = client.get(url);
    let mut headers = HeaderMap::new();
    add_headers(&mut headers, with_headers); 
    let resp = builder.headers(headers).send()?;
    let body = resp.text()?;
    Ok(body)
}

pub fn post_text(url: &str, text: String, with_headers: Option<HashMap<String, String>>) -> Result<String, LizError> {
    let client = reqwest::blocking::Client::new();
    let builder = client.post(url);
    let mut headers = HeaderMap::new();
    add_headers(&mut headers, with_headers); 
    let resp = builder.headers(headers).body(text).send()?;
    let body = resp.text()?;
    Ok(body)
}

fn add_headers(to: &mut HeaderMap, from: Option<HashMap<String, String>>) {
    to.insert("User-Agent", format!("Liz/{}", env!("CARGO_PKG_VERSION")).parse().unwrap());
    if let Some(from) = from {
        for (key, value) in from {
            if let Ok(name) = HeaderName::from_lowercase(key.as_bytes()) {
                to.insert(name, value.parse().unwrap());
            }
        }
    }
}
