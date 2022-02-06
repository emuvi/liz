use rlua::{Context, Table};

use crate::liz_texts;
use crate::utils;

use crate::LizError;

pub fn inject_texts<'a>(ctx: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let ask = ctx.create_function(|ctx, message: String| {
        utils::treat_error(ctx, liz_texts::ask(&message))
    })?;

    let ask_int = ctx.create_function(|ctx, message: String| {
        utils::treat_error(ctx, liz_texts::ask_int(&message))
    })?;

    let ask_float = ctx.create_function(|ctx, message: String| {
        utils::treat_error(ctx, liz_texts::ask_float(&message))
    })?;

    let ask_bool = ctx.create_function(|ctx, message: String| {
        utils::treat_error(ctx, liz_texts::ask_bool(&message))
    })?;

    let len = ctx.create_function(|_, text: String| Ok(liz_texts::len(&text)))?;

    let del = ctx.create_function(|_, (text, start, end): (String, usize, usize)| {
        Ok(liz_texts::del(&text, start, end))
    })?;

    let trim = ctx.create_function(|_, text: String| Ok(liz_texts::trim(&text)))?;

    let is_empty = ctx.create_function(|_, text: String| Ok(liz_texts::is_empty(&text)))?;

    let is_ascii = ctx.create_function(|_, text: String| Ok(liz_texts::is_ascii(&text)))?;

    let tolower = ctx.create_function(|_, text: String| Ok(liz_texts::tolower(&text)))?;

    let toupper = ctx.create_function(|_, text: String| Ok(liz_texts::toupper(&text)))?;

    let contains = ctx.create_function(|_, (text, part): (String, String)| {
        Ok(liz_texts::contains(&text, &part))
    })?;

    let find =
        ctx.create_function(|_, (text, part): (String, String)| Ok(liz_texts::find(&text, &part)))?;

    let rfind = ctx
        .create_function(|_, (text, part): (String, String)| Ok(liz_texts::rfind(&text, &part)))?;

    let starts_with = ctx.create_function(|_, (text, contents): (String, String)| {
        Ok(liz_texts::starts_with(&text, &contents))
    })?;

    let ends_with = ctx.create_function(|_, (text, contents): (String, String)| {
        Ok(liz_texts::ends_with(&text, &contents))
    })?;

    let split = ctx.create_function(|_, (text, pattern): (String, String)| {
        Ok(liz_texts::split(&text, &pattern))
    })?;

    let split_spaces = ctx.create_function(|_, text: String| Ok(liz_texts::split_spaces(&text)))?;

    let text_file_find = ctx.create_function(|ctx, (path, contents): (String, String)| {
        utils::treat_error(ctx, liz_texts::text_file_find(&path, &contents))
    })?;

    let text_file_find_any =
        ctx.create_function(|ctx, (path, contents): (String, Vec<String>)| {
            utils::treat_error(
                ctx,
                liz_texts::text_file_find_any(&path, contents.as_slice()),
            )
        })?;

    let text_files_find =
        ctx.create_function(|ctx, (paths, contents): (Vec<String>, String)| {
            utils::treat_error(ctx, liz_texts::text_files_find(paths, contents))
        })?;

    let text_files_find_any =
        ctx.create_function(|ctx, (paths, contents): (Vec<String>, Vec<String>)| {
            utils::treat_error(ctx, liz_texts::text_files_find_any(paths, contents))
        })?;

    let text_file_founds =
        ctx.create_function(|_, found: String| Ok(liz_texts::text_file_founds(&found)))?;

    let read =
        ctx.create_function(|ctx, path: String| utils::treat_error(ctx, liz_texts::read(&path)))?;

    let write = ctx.create_function(|ctx, (path, contents): (String, String)| {
        utils::treat_error(ctx, liz_texts::write(&path, &contents))
    })?;

    let append = ctx.create_function(|ctx, (path, contents): (String, String)| {
        utils::treat_error(ctx, liz_texts::append(&path, &contents))
    })?;

    let write_lines = ctx.create_function(|ctx, (path, lines): (String, Vec<String>)| {
        utils::treat_error(ctx, liz_texts::write_lines(&path, lines.as_slice()))
    })?;

    let append_lines = ctx.create_function(|ctx, (path, lines): (String, Vec<String>)| {
        utils::treat_error(ctx, liz_texts::append_lines(&path, lines.as_slice()))
    })?;

    liz.set("ask", ask)?;
    liz.set("ask_int", ask_int)?;
    liz.set("ask_float", ask_float)?;
    liz.set("ask_bool", ask_bool)?;
    liz.set("len", len)?;
    liz.set("del", del)?;
    liz.set("trim", trim)?;
    liz.set("is_empty", is_empty)?;
    liz.set("is_ascii", is_ascii)?;
    liz.set("tolower", tolower)?;
    liz.set("toupper", toupper)?;
    liz.set("contains", contains)?;
    liz.set("find", find)?;
    liz.set("rfind", rfind)?;
    liz.set("starts_with", starts_with)?;
    liz.set("ends_with", ends_with)?;
    liz.set("split", split)?;
    liz.set("split_spaces", split_spaces)?;
    liz.set("text_file_find", text_file_find)?;
    liz.set("text_file_find_any", text_file_find_any)?;
    liz.set("text_files_find", text_files_find)?;
    liz.set("text_files_find_any", text_files_find_any)?;
    liz.set("text_file_founds", text_file_founds)?;
    liz.set("read", read)?;
    liz.set("write", write)?;
    liz.set("append", append)?;
    liz.set("write_lines", write_lines)?;
    liz.set("append_lines", append_lines)?;

    Ok(())
}
