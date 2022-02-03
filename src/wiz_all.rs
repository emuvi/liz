use rlua::{Context, MultiValue, Value};

use std::path::Path;

use crate::liz_files;
use crate::utils;
use crate::wiz_codes;
use crate::wiz_execs;
use crate::wiz_files;
use crate::wiz_texts;
use crate::wiz_trans;

use crate::LizError;

pub fn inject_all(ctx: Context, path: impl AsRef<Path>, args: Option<Vec<String>>) -> Result<(), LizError> {
    let liz = ctx.create_table()?;
    liz.set("args", args)?;

    let rise_pwd = liz_files::pwd()?;
    liz.set("rise_pwd", rise_pwd)?;

    let rise_dir = liz_files::path_parent(&path)?;
    utils::put_stack_dir(&ctx, &liz, rise_dir.clone())?;
    liz.set("rise_dir", rise_dir)?;

    let rise_path = liz_files::path_absolute(&path)?;
    liz.set("rise_path", rise_path)?;

    let print_stack_dir = ctx.create_function(|ctx, ()| {
        utils::treat_error(ctx, utils::print_stack_dir(ctx))
    })?;

    let last_stack_dir = ctx.create_function(|ctx, ()| {
        utils::treat_error(ctx, utils::last_stack_dir(ctx))
    })?;

    let to_json_multi = ctx.create_function(|ctx, values: MultiValue| {
        utils::treat_error(ctx, utils::to_json_multi(values))
    })?;

    let to_json =
        ctx.create_function(|ctx, value: Value| utils::treat_error(ctx, utils::to_json(value)))?;

    let from_json = ctx.create_function(|ctx, source: String| {
        utils::treat_error(ctx, utils::from_json(ctx, source))
    })?;
    
    liz.set("print_stack_dir", print_stack_dir)?;
    liz.set("last_stack_dir", last_stack_dir)?;
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
