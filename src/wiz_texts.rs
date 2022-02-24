use rlua::{Context, Table};

use crate::liz_texts;
use crate::utils;

use crate::LizError;

pub fn inject_texts<'a>(lane: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let text = lane.create_function(|_, source: String| Ok(liz_texts::text(&source)))?;

    let ask =
        lane.create_function(|_, message: String| utils::treat_error(liz_texts::ask(&message)))?;

    let ask_int = lane
        .create_function(|_, message: String| utils::treat_error(liz_texts::ask_int(&message)))?;

    let ask_float = lane
        .create_function(|_, message: String| utils::treat_error(liz_texts::ask_float(&message)))?;

    let ask_bool = lane
        .create_function(|_, message: String| utils::treat_error(liz_texts::ask_bool(&message)))?;

    let len = lane.create_function(|_, text: String| Ok(liz_texts::len(&text)))?;

    let del = lane.create_function(|_, (text, start, end): (String, usize, usize)| {
        Ok(liz_texts::del(&text, start, end))
    })?;

    let trim = lane.create_function(|_, text: String| Ok(liz_texts::trim(&text)))?;

    let is_empty = lane.create_function(|_, text: String| Ok(liz_texts::is_empty(&text)))?;

    let is_ascii = lane.create_function(|_, text: String| Ok(liz_texts::is_ascii(&text)))?;

    let tolower = lane.create_function(|_, text: String| Ok(liz_texts::tolower(&text)))?;

    let toupper = lane.create_function(|_, text: String| Ok(liz_texts::toupper(&text)))?;

    let tocapital = lane.create_function(|_, text: String| Ok(liz_texts::tocapital(&text)))?;

    let contains = lane.create_function(|_, (text, part): (String, String)| {
        Ok(liz_texts::contains(&text, &part))
    })?;

    let find = lane
        .create_function(|_, (text, part): (String, String)| Ok(liz_texts::find(&text, &part)))?;

    let rfind = lane
        .create_function(|_, (text, part): (String, String)| Ok(liz_texts::rfind(&text, &part)))?;

    let starts_with = lane.create_function(|_, (text, contents): (String, String)| {
        Ok(liz_texts::starts_with(&text, &contents))
    })?;

    let ends_with = lane.create_function(|_, (text, contents): (String, String)| {
        Ok(liz_texts::ends_with(&text, &contents))
    })?;

    let split = lane.create_function(|_, (text, pattern): (String, String)| {
        Ok(liz_texts::split(&text, &pattern))
    })?;

    let split_spaces =
        lane.create_function(|_, text: String| Ok(liz_texts::split_spaces(&text)))?;

    let text_file_find = lane.create_function(|_, (path, contents): (String, String)| {
        utils::treat_error(liz_texts::text_file_find(&path, contents))
    })?;

    let text_file_find_any =
        lane.create_function(|_, (path, contents): (String, Vec<String>)| {
            utils::treat_error(liz_texts::text_file_find_any(&path, contents))
        })?;

    let text_files_find = lane.create_function(|_, (paths, contents): (Vec<String>, String)| {
        utils::treat_error(liz_texts::text_files_find(paths, contents))
    })?;

    let text_files_find_any =
        lane.create_function(|_, (paths, contents): (Vec<String>, Vec<String>)| {
            utils::treat_error(liz_texts::text_files_find_any(paths, contents))
        })?;

    let text_file_founds =
        lane.create_function(|_, found: String| Ok(liz_texts::text_file_founds(&found)))?;

    let read =
        lane.create_function(|_, path: String| utils::treat_error(liz_texts::read(&path)))?;

    let write = lane.create_function(|_, (path, contents): (String, String)| {
        utils::treat_error(liz_texts::write(&path, contents))
    })?;

    let append = lane.create_function(|_, (path, contents): (String, String)| {
        utils::treat_error(liz_texts::append(&path, contents))
    })?;

    let write_lines = lane.create_function(|_, (path, lines): (String, Vec<String>)| {
        utils::treat_error(liz_texts::write_lines(&path, lines))
    })?;

    let append_lines = lane.create_function(|_, (path, lines): (String, Vec<String>)| {
        utils::treat_error(liz_texts::append_lines(&path, lines))
    })?;

    liz.set("text", text)?;
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
    liz.set("tocapital", tocapital)?;
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
