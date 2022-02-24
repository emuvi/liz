use reqwest::header::HeaderMap;
use reqwest::header::HeaderName;

use std::collections::HashMap;

use crate::liz_debug::{dbg_err, dbg_stp};
use crate::LizError;

pub fn get(url: &str, with_headers: Option<HashMap<String, String>>) -> Result<String, LizError> {
    dbg_stp!(url, with_headers);
    let client = reqwest::blocking::Client::new();
    let builder = client.get(url);
    let mut headers = HeaderMap::new();
    add_headers(&mut headers, with_headers);
    let resp = builder
        .headers(headers)
        .send()
        .map_err(|err| dbg_err!(err))?;
    let body = resp.text().map_err(|err| dbg_err!(err))?;
    Ok(body)
}

pub fn post(
    url: &str,
    text: String,
    with_headers: Option<HashMap<String, String>>,
) -> Result<String, LizError> {
    dbg_stp!(url, text, with_headers);
    let client = reqwest::blocking::Client::new();
    let builder = client.post(url);
    let mut headers = HeaderMap::new();
    add_headers(&mut headers, with_headers);
    let resp = builder
        .headers(headers)
        .body(text)
        .send()
        .map_err(|err| dbg_err!(err))?;
    let body = resp.text().map_err(|err| dbg_err!(err))?;
    Ok(body)
}

pub fn download(
    origin: &str,
    destiny: &str,
    with_headers: Option<HashMap<String, String>>,
) -> Result<(), LizError> {
    dbg_stp!(origin, destiny, with_headers);
    let client = reqwest::blocking::Client::new();
    let builder = client.get(origin);
    let mut headers = HeaderMap::new();
    add_headers(&mut headers, with_headers);
    let resp = builder
        .headers(headers)
        .send()
        .map_err(|err| dbg_err!(err))?;
    let mut file = std::fs::File::create(destiny).map_err(|err| dbg_err!(err))?;
    let mut content = std::io::Cursor::new(resp.bytes().map_err(|err| dbg_err!(err))?);
    std::io::copy(&mut content, &mut file).map_err(|err| dbg_err!(err))?;
    Ok(())
}

fn add_headers(to: &mut HeaderMap, from: Option<HashMap<String, String>>) -> Result<(), LizError> {
    to.insert(
        "User-Agent",
        format!("Liz (Lua Wizard)/{}", env!("CARGO_PKG_VERSION"))
            .parse()
            .map_err(|err| dbg_err!(err))?,
    );
    if let Some(from) = from {
        for (key, value) in from {
            if let Ok(name) = HeaderName::from_lowercase(key.as_bytes()) {
                to.insert(name, value.parse().map_err(|err| dbg_err!(err))?);
            }
        }
    }
    Ok(())
}
