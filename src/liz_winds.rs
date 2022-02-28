use reqwest::blocking::Response;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderName;

use std::collections::HashMap;

use crate::liz_debug::{self, dbg_err, dbg_step};
use crate::LizError;

pub fn get(url: &str, with_headers: Option<HashMap<String, String>>) -> Result<String, LizError> {
    dbg_step!(url, with_headers);
    let client = reqwest::blocking::Client::new();
    let builder = client.get(url);
    let mut headers = HeaderMap::new();
    add_headers(&mut headers, with_headers).map_err(|err| dbg_err!(err))?;
    let resp = builder
        .headers(headers)
        .send()
        .map_err(|err| dbg_err!(err))?;
    treat_response(&resp).map_err(|err| dbg_err!(err))?;
    let body = resp.text().map_err(|err| dbg_err!(err))?;
    Ok(body)
}

pub fn post(
    url: &str,
    text: String,
    with_headers: Option<HashMap<String, String>>,
) -> Result<String, LizError> {
    dbg_step!(url, text, with_headers);
    let client = reqwest::blocking::Client::new();
    let builder = client.post(url);
    let mut headers = HeaderMap::new();
    add_headers(&mut headers, with_headers).map_err(|err| dbg_err!(err))?;
    let resp = builder
        .headers(headers)
        .body(text)
        .send()
        .map_err(|err| dbg_err!(err))?;
    treat_response(&resp).map_err(|err| dbg_err!(err))?;
    let body = resp.text().map_err(|err| dbg_err!(err))?;
    Ok(body)
}

pub fn download(
    origin: &str,
    destiny: &str,
    with_headers: Option<HashMap<String, String>>,
) -> Result<(), LizError> {
    dbg_step!(origin, destiny, with_headers);
    let client = reqwest::blocking::Client::new();
    let builder = client.get(origin);
    let mut headers = HeaderMap::new();
    add_headers(&mut headers, with_headers).map_err(|err| dbg_err!(err))?;
    let resp = builder
        .headers(headers)
        .send()
        .map_err(|err| dbg_err!(err))?;
    treat_response(&resp).map_err(|err| dbg_err!(err))?;
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

fn treat_response(resp: &Response) -> Result<(), LizError> {
    if !resp.status().is_success() {
        return Err(liz_debug::wrong(format!(
            "Response Error: {}",
            resp.status()
        )));
    }
    Ok(())
}
