use rlua::{Context, Lua, MultiValue, Table};

use std::error::Error;

use liz_debug::{dbg_bub, dbg_err, dbg_inf, dbg_knd, dbg_stp};

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
    dbg_stp!(path, args);
    let handler = rise(path, args).map_err(|err| dbg_bub!(err))?;
    race(path, &handler).map_err(|err| dbg_bub!(err))
}

pub fn rise(path: &str, args: &Option<Vec<String>>) -> Result<Lua, LizError> {
    dbg_inf!("Rising", path, args);
    let handler = Lua::new();
    let mut error: Option<LizError> = None;
    handler.context(|lane| {
        if let Err(err) = wiz_all::inject_all(lane, path, args) {
            error = Some(err);
        }
    });
    if let Some(err) = error {
        return Err(dbg_err!(err, path, args));
    }
    Ok(handler)
}

pub fn race(path: &str, handler: &Lua) -> Result<Vec<String>, LizError> {
    dbg_inf!("Racing", path);
    let mut result: Option<Result<Vec<String>, LizError>> = None;
    handler.context(|lane| result = Some(race_in(lane, path)));
    if result.is_none() {
        dbg_knd!("WARN", "Could not reach a result", &path);
        return Err(dbg_err!("Could not reach a result", path));
    }
    let result = result.unwrap();
    result
}

pub fn race_in(lane: Context, path: &str) -> Result<Vec<String>, LizError> {
    dbg_stp!(path);
    let globals = lane.globals();
    let liz: Table = globals.get("liz").map_err(|err| dbg_err!(err))?;

    let path = liz_codes::liz_suit_path(path).map_err(|err| dbg_bub!(err))?;
    dbg_stp!(path);

    let path = if liz_paths::is_symlink(&path) {
        liz_paths::path_walk(&path).map_err(|err| dbg_bub!(err, path))?
    } else {
        path
    };
    dbg_stp!(path);

    let path = if liz_paths::is_relative(&path) {
        let stack_dir = utils::get_stack_dir(&liz).map_err(|err| dbg_bub!(err))?;
        liz_paths::path_join(&stack_dir, &path).map_err(|err| dbg_bub!(err))?
    } else {
        path
    };
    dbg_stp!(path);

    let race_pwd = liz_paths::wd().map_err(|err| dbg_bub!(err))?;
    dbg_stp!(race_pwd);

    let race_dir = liz_paths::path_parent(&path).map_err(|err| dbg_bub!(err))?;
    dbg_stp!(race_dir);
    utils::put_stack_dir(&lane, &liz, race_dir.clone()).map_err(|err| dbg_bub!(err))?;

    let race_path = path;
    dbg_stp!(race_path);

    liz.set("race_pwd", race_pwd).map_err(|err| dbg_err!(err))?;
    liz.set("race_dir", race_dir).map_err(|err| dbg_err!(err))?;
    liz.set("race_path", race_path.clone())
        .map_err(|err| dbg_err!(err))?;

    liz_codes::gotta_lizs(&race_path).map_err(|err| dbg_bub!(err))?;

    let source = std::fs::read_to_string(race_path).map_err(|err| dbg_err!(err))?;
    let values = eval_in(lane, source).map_err(|err| dbg_bub!(err))?;
    utils::pop_stack_dir(&liz).map_err(|err| dbg_bub!(err))?;
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
