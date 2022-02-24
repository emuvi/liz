use rlua::{Context, Lua, MultiValue, Table};

use std::error::Error;

use liz_debug::{dbg_err, dbg_inf, dbg_knd, dbg_stp};

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
    let handler = rise(path, args).map_err(|err| dbg_err!(err, path, args))?;
    race(path, &handler).map_err(|err| dbg_err!(err, path, args))
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
    result.map_err(|err| dbg_err!(err, path))
}

pub fn race_in(lane: Context, path: &str) -> Result<Vec<String>, LizError> {
    dbg_stp!(path);
    let globals = lane.globals();
    let liz: Table = globals.get("liz")?;

    let path = utils::liz_suit_path(path)?;
    dbg_stp!(path);
    let path = if liz_paths::is_relative(&path) {
        let stack_dir = utils::get_stack_dir(&liz)?;
        liz_paths::path_join(&stack_dir, &path)?
    } else {
        path
    };
    dbg_stp!(path);

    let race_pwd = liz_paths::pwd()?;
    dbg_stp!(race_pwd);
    liz.set("race_pwd", race_pwd)?;

    let race_dir = liz_paths::path_parent(&path)?;
    dbg_stp!(race_dir);
    utils::put_stack_dir(&lane, &liz, race_dir.clone())?;
    liz.set("race_dir", race_dir)?;

    let race_path = liz_paths::path_absolute(&path)?;
    dbg_stp!(race_path);
    liz.set("race_path", race_path.clone())?;

    let source = get_source(&race_path)?;
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

fn get_source(path: &str) -> Result<String, LizError> {
    let sep = liz_paths::os_sep();
    let lizs_dir = format!("{}lizs{}", sep, sep);
    let lizs_pos = path.find(&lizs_dir);
    if let Some(lizs_pos) = lizs_pos {
        if !liz_paths::has(path) {
            let path_dir = liz_paths::path_parent(path)?;
            std::fs::create_dir_all(path_dir)?;
            let lizs_path = &path[lizs_pos+lizs_dir.len()..];
            let lizs_path = lizs_path.replace("\\", "/");
            let origin = format!("https://raw.githubusercontent.com/emuvi/lizs/main/{}", &lizs_path);
            liz_winds::download(&origin, path, None)?;
        }
    }
    Ok(std::fs::read_to_string(path)?)
}
