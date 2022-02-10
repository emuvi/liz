use rlua::{Context, Table};

use std::collections::HashMap;

use crate::liz_trans;
use crate::utils;

use crate::LizError;

pub fn inject_trans<'a>(lane: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let get = lane.create_function(
        |lane, (url, headers): (String, Option<HashMap<String, String>>)| {
            utils::treat_error(lane, liz_trans::get(&url, headers))
        },
    )?;

    let post = lane.create_function(
        |lane, (url, text, headers): (String, String, Option<HashMap<String, String>>)| {
            utils::treat_error(lane, liz_trans::post(&url, text, headers))
        },
    )?;

    liz.set("get", get)?;
    liz.set("post", post)?;

    Ok(())
}
