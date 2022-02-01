use rlua::{Context, MultiValue, Value};

use crate::utils;
use crate::wiz_codes;
use crate::wiz_execs;
use crate::wiz_files;
use crate::wiz_texts;
use crate::wiz_trans;

use crate::LizError;

pub fn inject_all(ctx: Context, args: Option<Vec<String>>) -> Result<(), LizError> {
    let liz = ctx.create_table()?;
    liz.set("args", args)?;

    let path = std::env::current_dir()?;
    let path_display = path
        .to_str()
        .ok_or("Could not get the display path of the rise.")?;
    liz.set("rise_dir", String::from(path_display))?;

    let to_json_multi = ctx.create_function(|ctx, values: MultiValue| {
        utils::treat_error(ctx, utils::to_json_multi(values))
    })?;

    let to_json =
        ctx.create_function(|ctx, value: Value| utils::treat_error(ctx, utils::to_json(value)))?;

    let from_json = ctx.create_function(|ctx, source: String| {
        utils::treat_error(ctx, utils::from_json(ctx, source))
    })?;
    liz.set("to_json_multi", to_json_multi)?;
    liz.set("to_json", to_json)?;
    liz.set("from_json", from_json)?;

    wiz_codes::inject_codes(ctx, &liz)?;
    wiz_execs::inject_execs(ctx, &liz)?;
    wiz_files::inject_files(ctx, &liz)?;
    wiz_texts::inject_texts(ctx, &liz)?;
    wiz_trans::inject_trans(ctx, &liz)?;

    let globals = ctx.globals();
    globals.set("liz", liz)?;

    Ok(())
}
