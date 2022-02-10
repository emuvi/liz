pub mod liz_codes;
pub mod liz_execs;
pub mod liz_forms;
pub mod liz_paths;
pub mod liz_texts;
pub mod liz_trans;

pub mod utils;

mod wiz_all;
mod wiz_codes;
mod wiz_execs;
mod wiz_paths;
mod wiz_texts;
mod wiz_trans;

use rlua::{Context, Lua, MultiValue, Table};

use std::error::Error;

use utils::debug;

pub type LizError = Box<dyn Error + Send + Sync>;

pub fn run(path: &str, args: Option<Vec<String>>) -> Result<Vec<String>, LizError> {
    let handler = rise(path, args).map_err(|err| debug!(err, "rise", path))?;
    race(path, &handler).map_err(|err| debug!(err, "race", path))
}

pub fn rise(path: &str, args: Option<Vec<String>>) -> Result<Lua, LizError> {
    let handler = Lua::new();
    let mut error: Option<LizError> = None;
    handler.context(|lane| {
        if let Err(err) = wiz_all::inject_all(lane, path, args) {
            error = Some(err);
        }
    });
    if let Some(err) = error {
        return Err(debug!(err, "inject_all", path));
    }
    Ok(handler)
}

pub fn race(path: &str, handler: &Lua) -> Result<Vec<String>, LizError> {
    let mut result: Option<Result<Vec<String>, LizError>> = None;
    handler.context(|lane| result = Some(race_in(lane, path)));
    if result.is_none() {
        return Err(debug!("Could not reach a result", "is_none", path));
    }
    let result = result.unwrap();
    result.map_err(|err| debug!(err, "race_in", path))
}

pub fn race_in(lane: Context, path: &str) -> Result<Vec<String>, LizError> {
    let globals = lane.globals();
    let liz: Table = globals.get("liz")?;

    let path = utils::add_liz_extension(path);
    let path = if liz_paths::is_relative(&path) {
        let stack_dir = utils::get_stack_dir(&liz)?;
        liz_paths::path_join(&stack_dir, &path)?
    } else {
        path
    };

    let race_pwd = liz_paths::pwd()?;
    liz.set("race_pwd", race_pwd)?;

    let race_dir = liz_paths::path_parent(&path)?;
    utils::put_stack_dir(&lane, &liz, race_dir.clone())?;
    liz.set("race_dir", race_dir)?;

    let race_path = liz_paths::path_absolute(&path)?;
    liz.set("race_path", race_path.clone())?;

    let source = std::fs::read_to_string(race_path)?;
    let values = eval_in(lane, &source)?;
    utils::pop_stack_dir(&liz)?;
    Ok(values)
}

pub fn eval_in(lane: Context, source: &str) -> Result<Vec<String>, LizError> {
    let mut source = source.trim();
    if source.starts_with("#!") {
        if let Some(first_line) = source.find("\n") {
            source = (&source[first_line + 1..]).trim();
        }
    }
    let values = lane.load(source).eval::<MultiValue>()?;
    utils::to_json_multi(values)
}
