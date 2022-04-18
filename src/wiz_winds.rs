use rlua::{Context, Table};
use rubx::rux_winds;

use std::collections::HashMap;

use crate::utils;
use crate::LizError;

pub fn inject_winds<'a>(lane: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let get = lane.create_function(
        |_, (url, headers): (String, Option<HashMap<String, String>>)| {
            utils::treat_error(rux_winds::get(&url, headers))
        },
    )?;

    let post = lane.create_function(
        |_, (url, text, headers): (String, String, Option<HashMap<String, String>>)| {
            utils::treat_error(rux_winds::post(&url, text, headers))
        },
    )?;

    let download = lane.create_function(
        |_, (origin, destiny, headers): (String, String, Option<HashMap<String, String>>)| {
            utils::treat_error(rux_winds::download(&origin, &destiny, headers))
        },
    )?;

    liz.set("get", get)?;
    liz.set("post", post)?;
    liz.set("download", download)?;

    Ok(())
}
