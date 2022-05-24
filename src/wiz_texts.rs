use rlua::{Context, Table};
use rubx::rux_texts;

use crate::utils;

use crate::LizError;

pub fn inject_texts<'a>(lane: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
  let ask =
    lane.create_function(|_, message: String| utils::treat_error(rux_texts::ask(&message)))?;

  let ask_int = lane
    .create_function(|_, message: String| utils::treat_error(rux_texts::ask_int(&message)))?;

  let ask_float = lane
    .create_function(|_, message: String| utils::treat_error(rux_texts::ask_float(&message)))?;

  let ask_bool = lane
    .create_function(|_, message: String| utils::treat_error(rux_texts::ask_bool(&message)))?;

  let len = lane.create_function(|_, text: String| Ok(rux_texts::len(&text)))?;

  let del = lane.create_function(|_, (text, start, end): (String, usize, usize)| {
    Ok(rux_texts::del(&text, start, end))
  })?;

  let del_rex = lane.create_function(|_, (text, regex): (String, String)| {
    utils::treat_error(rux_texts::del_rex(&text, &regex))
  })?;

  let trim = lane.create_function(|_, text: String| Ok(rux_texts::trim(&text)))?;

  let is_empty = lane.create_function(|_, text: String| Ok(rux_texts::is_empty(&text)))?;

  let is_ascii = lane.create_function(|_, text: String| Ok(rux_texts::is_ascii(&text)))?;

  let is_equals = lane.create_function(|_, (text, with): (String, String)| {
    Ok(rux_texts::is_equals(&text, &with))
  })?;

  let is_equally = lane.create_function(|_, (text, with): (String, String)| {
    Ok(rux_texts::is_equally(&text, &with))
  })?;

  let is_likely = lane.create_function(|_, (text, with): (String, String)| {
    Ok(rux_texts::is_likely(&text, &with))
  })?;

  let is_whitespace =
    lane.create_function(|_, text: String| Ok(rux_texts::is_whitespace(&text)))?;

  let is_linespace =
    lane.create_function(|_, text: String| Ok(rux_texts::is_linespace(&text)))?;

  let is_linebreak =
    lane.create_function(|_, text: String| Ok(rux_texts::is_linebreak(&text)))?;

  let is_brackets =
    lane.create_function(|_, text: String| Ok(rux_texts::is_brackets(&text)))?;

  let is_quotation =
    lane.create_function(|_, text: String| Ok(rux_texts::is_quotation(&text)))?;

  let tolower = lane.create_function(|_, text: String| Ok(rux_texts::tolower(&text)))?;

  let toupper = lane.create_function(|_, text: String| Ok(rux_texts::toupper(&text)))?;

  let tocapital = lane.create_function(|_, text: String| Ok(rux_texts::tocapital(&text)))?;

  let contains = lane.create_function(|_, (text, part): (String, String)| {
    Ok(rux_texts::contains(&text, &part))
  })?;

  let find = lane
    .create_function(|_, (text, part): (String, String)| Ok(rux_texts::find(&text, &part)))?;

  let rfind = lane
    .create_function(|_, (text, part): (String, String)| Ok(rux_texts::rfind(&text, &part)))?;

  let starts_with = lane.create_function(|_, (text, contents): (String, String)| {
    Ok(rux_texts::starts_with(&text, &contents))
  })?;

  let ends_with = lane.create_function(|_, (text, contents): (String, String)| {
    Ok(rux_texts::ends_with(&text, &contents))
  })?;

  let split = lane.create_function(|_, (text, pattern): (String, String)| {
    Ok(rux_texts::split(&text, &pattern))
  })?;

  let split_spaces =
    lane.create_function(|_, text: String| Ok(rux_texts::split_spaces(&text)))?;

  let text_file_find = lane.create_function(|_, (path, contents): (String, String)| {
    utils::treat_error(rux_texts::text_file_find(&path, contents))
  })?;

  let text_file_find_any =
    lane.create_function(|_, (path, contents): (String, Vec<String>)| {
      utils::treat_error(rux_texts::text_file_find_any(&path, contents))
    })?;

  let text_files_find =
    lane.create_function(|_, (paths, contents): (Vec<String>, String)| {
      utils::treat_error(rux_texts::text_files_find(paths, contents))
    })?;

  let text_files_find_any =
    lane.create_function(|_, (paths, contents): (Vec<String>, Vec<String>)| {
      utils::treat_error(rux_texts::text_files_find_any(paths, contents))
    })?;

  let text_file_founds =
    lane.create_function(|_, found: String| Ok(rux_texts::text_file_founds(&found)))?;

  let read =
    lane.create_function(|_, path: String| utils::treat_error(rux_texts::read(&path)))?;

  let write = lane.create_function(|_, (path, contents): (String, String)| {
    utils::treat_error(rux_texts::write(&path, contents))
  })?;

  let append = lane.create_function(|_, (path, contents): (String, String)| {
    utils::treat_error(rux_texts::append(&path, contents))
  })?;

  let write_lines = lane.create_function(|_, (path, lines): (String, Vec<String>)| {
    utils::treat_error(rux_texts::write_lines(&path, lines))
  })?;

  let write_inputs = lane
    .create_function(|_, path: String| utils::treat_error(rux_texts::write_inputs(&path)))?;

  let append_lines = lane.create_function(|_, (path, lines): (String, Vec<String>)| {
    utils::treat_error(rux_texts::append_lines(&path, lines.as_slice()))
  })?;

  let append_inputs = lane
    .create_function(|_, path: String| utils::treat_error(rux_texts::append_inputs(&path)))?;

  let find_bigger_line = lane.create_function(|_, lines: Vec<String>| {
    Ok(rux_texts::find_bigger_line(&lines.as_slice()).map(String::from))
  })?;

  let find_smaller_line = lane.create_function(|_, lines: Vec<String>| {
    Ok(rux_texts::find_smaller_line(&lines.as_slice()).map(String::from))
  })?;

  let read_setup =
    lane.create_function(|_, path: String| utils::treat_error(rux_texts::read_setup(&path)))?;

  let is_truthy = lane.create_function(|_, value: String| Ok(rux_texts::is_truthy(&value)))?;

  liz.set("ask", ask)?;
  liz.set("ask_int", ask_int)?;
  liz.set("ask_float", ask_float)?;
  liz.set("ask_bool", ask_bool)?;
  liz.set("len", len)?;
  liz.set("del", del)?;
  liz.set("del_rex", del_rex)?;
  liz.set("trim", trim)?;
  liz.set("is_empty", is_empty)?;
  liz.set("is_ascii", is_ascii)?;
  liz.set("is_equals", is_equals)?;
  liz.set("is_equally", is_equally)?;
  liz.set("is_likely", is_likely)?;
  liz.set("is_whitespace", is_whitespace)?;
  liz.set("is_linespace", is_linespace)?;
  liz.set("is_linebreak", is_linebreak)?;
  liz.set("is_brackets", is_brackets)?;
  liz.set("is_quotation", is_quotation)?;
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
  liz.set("write_inputs", write_inputs)?;
  liz.set("append_lines", append_lines)?;
  liz.set("append_inputs", append_inputs)?;
  liz.set("find_bigger_line", find_bigger_line)?;
  liz.set("find_smaller_line", find_smaller_line)?;
  liz.set("read_setup", read_setup)?;
  liz.set("is_truthy", is_truthy)?;

  Ok(())
}
