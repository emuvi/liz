use rlua::{Context, Table};

use std::collections::HashMap;

use crate::trans;
use crate::utils;

use crate::LizError;

pub fn inject_trans<'a>(ctx: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let get = ctx.create_function(
        |ctx, (url, headers): (String, Option<HashMap<String, String>>)| {
            utils::treat_error(ctx, trans::get(&url, headers))
        },
    )?;

    let post = ctx.create_function(
        |ctx, (url, text, headers): (String, String, Option<HashMap<String, String>>)| {
            utils::treat_error(ctx, trans::post(&url, text, headers))
        },
    )?;

    liz.set("get", get)?;
    liz.set("post", post)?;

    Ok(())
}