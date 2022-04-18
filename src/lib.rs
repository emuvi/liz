use rlua::{Context, Lua, MultiValue, Table};
use rubx::rux_paths;
use rubx::{rux_dbg_bleb, rux_dbg_erro, rux_dbg_info, rux_dbg_jolt};
use rubx::{rux_dbg_call, rux_dbg_reav, rux_dbg_step};

use std::error::Error;

pub mod liz_codes;
pub mod liz_fires;
pub mod liz_forms;
pub mod liz_group;
pub mod liz_logic;
pub mod liz_parse;

mod tests;
mod utils;

mod wiz_all;
mod wiz_codes;
mod wiz_fires;
mod wiz_forms;
mod wiz_group;
mod wiz_logic;
mod wiz_parse;
mod wiz_paths;
mod wiz_texts;
mod wiz_times;
mod wiz_winds;

pub type LizError = Box<dyn Error + Send + Sync>;

pub fn run(path: &str, args: &Option<Vec<String>>) -> Result<Vec<String>, LizError> {
    rux_dbg_call!(path, args);
    let (rise_path, handler) = rise(path, args).map_err(|err| rux_dbg_bleb!(err))?;
    rux_dbg_step!(rise_path);
    rux_dbg_reav!(race(&rise_path, &handler).map_err(|err| rux_dbg_bleb!(err)));
}

pub fn rise(path: &str, args: &Option<Vec<String>>) -> Result<(String, Lua), LizError> {
    rux_dbg_call!(path, args);
    rux_dbg_info!("Rising a new lane", path, args);
    let handler = Lua::new();
    let mut rise_path: Option<String> = None;
    let mut rise_error: Option<LizError> = None;
    handler.context(|lane| match wiz_all::inject_all(lane, path, args) {
        Ok(path) => (rise_path = Some(path)),
        Err(error) => (rise_error = Some(error)),
    });
    if let Some(err) = rise_error {
        return Err(rux_dbg_bleb!(err));
    }
    let rise_path = rise_path
        .ok_or("We should have reach the rise path")
        .map_err(|err| rux_dbg_jolt!("WARN", err))?;
    Ok((rise_path, handler))
}

pub fn race(path: &str, handler: &Lua) -> Result<Vec<String>, LizError> {
    rux_dbg_call!(path);
    rux_dbg_info!("Racing the path on the lane", path);
    let mut result: Option<Result<Vec<String>, LizError>> = None;
    handler.context(|lane| result = Some(race_in(lane, path)));
    if result.is_none() {
        rux_dbg_reav!(Err(rux_dbg_jolt!("WARN", "Could not reach a result", path)));
    }
    rux_dbg_reav!(result.unwrap());
}

pub fn race_in(lane: Context, path: &str) -> Result<Vec<String>, LizError> {
    rux_dbg_call!(path);
    let globals = lane.globals();
    let liz: Table = globals.get("Liz").map_err(|err| rux_dbg_erro!(err))?;

    let suit_path = liz_codes::liz_suit_path(path).map_err(|err| rux_dbg_bleb!(err))?;
    rux_dbg_step!(suit_path);

    let suit_path = if rux_paths::is_relative(&suit_path) {
        let stack_dir = utils::liz_stacked_dir(&liz).map_err(|err| rux_dbg_bleb!(err))?;
        rux_paths::path_join(&stack_dir, &suit_path).map_err(|err| rux_dbg_bleb!(err))?
    } else {
        suit_path
    };
    rux_dbg_step!(suit_path);

    let race_wd = rux_paths::wd().map_err(|err| rux_dbg_bleb!(err))?;
    rux_dbg_step!(race_wd);

    let race_dir = rux_paths::path_parent(&suit_path).map_err(|err| rux_dbg_bleb!(err))?;
    rux_dbg_step!(race_dir);
    utils::put_stack_dir(&lane, &liz, race_dir.clone()).map_err(|err| rux_dbg_bleb!(err))?;

    let race_path = rux_paths::path_absolute(&suit_path).map_err(|err| rux_dbg_bleb!(err))?;
    rux_dbg_step!(race_path);

    liz.set("race_wd", race_wd).map_err(|err| rux_dbg_erro!(err))?;
    liz.set("race_dir", race_dir)
        .map_err(|err| rux_dbg_erro!(err))?;
    liz.set("race_path", race_path.clone())
        .map_err(|err| rux_dbg_erro!(err))?;

    liz_codes::gotta_lizs(&race_path).map_err(|err| rux_dbg_bleb!(err))?;

    let source = std::fs::read_to_string(race_path).map_err(|err| rux_dbg_erro!(err))?;
    rux_dbg_step!(source);
    let values = eval_in(lane, source).map_err(|err| rux_dbg_bleb!(err))?;
    rux_dbg_step!(values);
    utils::pop_stack_dir(&liz).map_err(|err| rux_dbg_bleb!(err))?;
    rux_dbg_reav!(Ok(values));
}

pub fn eval_in(lane: Context, source: String) -> Result<Vec<String>, LizError> {
    rux_dbg_call!(source);
    let mut source = source.trim();
    if source.starts_with("#!") {
        if let Some(first_line) = source.find("\n") {
            source = (&source[first_line + 1..]).trim();
        }
    }
    rux_dbg_step!(source);
    let values = lane
        .load(source)
        .eval::<MultiValue>()
        .map_err(|err| rux_dbg_erro!(err))?;
    rux_dbg_reav!(utils::to_json_multi(values));
}
