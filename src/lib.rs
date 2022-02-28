use rlua::{Context, Lua, MultiValue, Table};

use std::error::Error;

use liz_debug::{dbg_ebb, dbg_call, dbg_err, dbg_inf, dbg_step, dbg_trw};

pub mod liz_codes;
pub mod liz_debug;
pub mod liz_fires;
pub mod liz_forms;
pub mod liz_parse;
pub mod liz_paths;
pub mod liz_texts;
pub mod liz_times;
pub mod liz_winds;

mod tst_paths;

mod utils;

mod wiz_all;
mod wiz_codes;
mod wiz_fires;
mod wiz_paths;
mod wiz_texts;
mod wiz_times;
mod wiz_winds;

pub type LizError = Box<dyn Error + Send + Sync>;

pub fn run(path: &str, args: &Option<Vec<String>>) -> Result<Vec<String>, LizError> {
    dbg_call!(path, args);
    let (rise_path, handler) = rise(path, args).map_err(|err| dbg_ebb!(err))?;
    race(&rise_path, &handler).map_err(|err| dbg_ebb!(err))
}

pub fn rise(path: &str, args: &Option<Vec<String>>) -> Result<(String, Lua), LizError> {
    dbg_inf!("Rising", path, args);
    let handler = Lua::new();
    let mut rise_path: Option<String> = None;
    let mut rise_error: Option<LizError> = None;
    handler.context(|lane| match wiz_all::inject_all(lane, path, args) {
        Ok(path) => (rise_path = Some(path)),
        Err(error) => (rise_error = Some(error)),
    });
    if let Some(err) = rise_error {
        return Err(dbg_ebb!(err));
    }
    let rise_path = rise_path
        .ok_or("We should have reach the rise path")
        .map_err(|err| dbg_trw!("WARN", err))?;
    Ok((rise_path, handler))
}

pub fn race(path: &str, handler: &Lua) -> Result<Vec<String>, LizError> {
    dbg_inf!("Racing", path);
    let mut result: Option<Result<Vec<String>, LizError>> = None;
    handler.context(|lane| result = Some(race_in(lane, path)));
    if result.is_none() {
        return Err(dbg_trw!("WARN", "Could not reach a result", path));
    }
    let result = result.unwrap();
    result
}

pub fn race_in(lane: Context, path: &str) -> Result<Vec<String>, LizError> {
    dbg_step!(path);
    let globals = lane.globals();
    let liz: Table = globals.get("liz").map_err(|err| dbg_err!(err))?;

    let suit_path = liz_codes::liz_suit_path(path).map_err(|err| dbg_ebb!(err))?;
    dbg_step!(suit_path);

    let suit_path = if liz_paths::is_relative(&suit_path) {
        let stack_dir = utils::get_stack_dir(&liz).map_err(|err| dbg_ebb!(err))?;
        liz_paths::path_join(&stack_dir, &suit_path).map_err(|err| dbg_ebb!(err))?
    } else {
        suit_path
    };
    dbg_step!(suit_path);

    let race_wd = liz_paths::wd().map_err(|err| dbg_ebb!(err))?;
    dbg_step!(race_wd);

    let race_dir = liz_paths::path_parent(&suit_path).map_err(|err| dbg_ebb!(err))?;
    dbg_step!(race_dir);
    utils::put_stack_dir(&lane, &liz, race_dir.clone()).map_err(|err| dbg_ebb!(err))?;

    let race_path = liz_paths::path_absolute(&suit_path).map_err(|err| dbg_ebb!(err))?;
    dbg_step!(race_path);

    liz.set("race_wd", race_wd).map_err(|err| dbg_err!(err))?;
    liz.set("race_dir", race_dir).map_err(|err| dbg_err!(err))?;
    liz.set("race_path", race_path.clone())
        .map_err(|err| dbg_err!(err))?;

    liz_codes::gotta_lizs(&race_path).map_err(|err| dbg_ebb!(err))?;

    let source = std::fs::read_to_string(race_path).map_err(|err| dbg_err!(err))?;
    dbg_step!(source);
    let values = eval_in(lane, source).map_err(|err| dbg_ebb!(err))?;
    dbg_step!(values);
    utils::pop_stack_dir(&liz).map_err(|err| dbg_ebb!(err))?;
    Ok(values)
}

pub fn eval_in(lane: Context, source: String) -> Result<Vec<String>, LizError> {
    let mut source = source.trim();
    if source.starts_with("#!") {
        if let Some(first_line) = source.find("\n") {
            source = (&source[first_line + 1..]).trim();
        }
    }
    let values = lane
        .load(source)
        .eval::<MultiValue>()
        .map_err(|err| dbg_err!(err))?;
    utils::to_json_multi(values)
}
