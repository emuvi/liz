use rlua::{Context, Table};

use std::collections::HashMap;

use crate::liz_winds;
use crate::utils;

use crate::LizError;

pub fn inject_trans<'a>(lane: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let get = lane.create_function(
        |lane, (url, headers): (String, Option<HashMap<String, String>>)| {
            utils::treat_error(lane, liz_winds::get(&url, headers))
        },
    )?;

    let post = lane.create_function(
        |lane, (url, text, headers): (String, String, Option<HashMap<String, String>>)| {
            utils::treat_error(lane, liz_winds::post(&url, text, headers))
        },
    )?;

    let download = lane.create_function(
        |lane, (origin, destiny, headers): (String, String, Option<HashMap<String, String>>)| {
            utils::treat_error(lane, liz_winds::download(&origin, &destiny, headers))
        },
    )?;

    liz.set("get", get)?;
    liz.set("post", post)?;
    liz.set("download", download)?;

    Ok(())
}
