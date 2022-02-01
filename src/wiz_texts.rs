use rlua::{Context, Table};

use crate::texts;
use crate::utils;

use crate::LizError;

pub fn inject_texts<'a>(ctx: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let ask =
        ctx.create_function(|ctx, message: String| utils::treat_error(ctx, texts::ask(&message)))?;

    let ask_int = ctx.create_function(|ctx, message: String| {
        utils::treat_error(ctx, texts::ask_int(&message))
    })?;

    let ask_float = ctx.create_function(|ctx, message: String| {
        utils::treat_error(ctx, texts::ask_float(&message))
    })?;

    let ask_bool = ctx.create_function(|ctx, message: String| {
        utils::treat_error(ctx, texts::ask_bool(&message))
    })?;

    let trim = ctx.create_function(|_, text: String| Ok(texts::trim(&text)))?;

    let is_empty = ctx.create_function(|_, text: String| Ok(texts::is_empty(&text)))?;

    let is_ascii = ctx.create_function(|_, text: String| Ok(texts::is_ascii(&text)))?;

    let tolower = ctx.create_function(|_, text: String| Ok(texts::tolower(&text)))?;

    let toupper = ctx.create_function(|_, text: String| Ok(texts::toupper(&text)))?;

    let contains =
        ctx.create_function(|_, (text, part): (String, String)| Ok(texts::contains(&text, &part)))?;

    let find =
        ctx.create_function(|_, (text, part): (String, String)| Ok(texts::find(&text, &part)))?;

    let starts_with = ctx.create_function(|_, (text, contents): (String, String)| {
        Ok(texts::starts_with(&text, &contents))
    })?;

    let ends_with = ctx.create_function(|_, (text, contents): (String, String)| {
        Ok(texts::ends_with(&text, &contents))
    })?;

    let text_path_find = ctx.create_function(|ctx, (path, contents): (String, String)| {
        utils::treat_error(ctx, texts::text_path_find(&path, &contents))
    })?;

    let text_dir_find = ctx.create_function(|ctx, (path, contents): (String, String)| {
        utils::treat_error(ctx, texts::text_dir_find(&path, &contents))
    })?;

    let text_file_find = ctx.create_function(|ctx, (path, contents): (String, String)| {
        utils::treat_error(ctx, texts::text_file_find(&path, &contents))
    })?;

    let text_files_find =
        ctx.create_function(|ctx, (paths, contents): (Vec<String>, String)| {
            utils::treat_error(ctx, texts::text_files_find(paths, contents))
        })?;

    liz.set("ask", ask)?;
    liz.set("ask_int", ask_int)?;
    liz.set("ask_float", ask_float)?;
    liz.set("ask_bool", ask_bool)?;
    liz.set("trim", trim)?;
    liz.set("is_empty", is_empty)?;
    liz.set("is_ascii", is_ascii)?;
    liz.set("tolower", tolower)?;
    liz.set("toupper", toupper)?;
    liz.set("contains", contains)?;
    liz.set("find", find)?;
    liz.set("starts_with", starts_with)?;
    liz.set("ends_with", ends_with)?;
    liz.set("text_path_find", text_path_find)?;
    liz.set("text_dir_find", text_dir_find)?;
    liz.set("text_file_find", text_file_find)?;
    liz.set("text_files_find", text_files_find)?;

    Ok(())
}
