use rlua::{Context, Lua, MultiValue, Table};

use std::error::Error;

pub mod liz_codes;
pub mod liz_execs;
pub mod liz_files;
pub mod liz_slabs;
pub mod liz_texts;
pub mod liz_trans;

pub mod utils;

mod wiz_all;
mod wiz_codes;
mod wiz_execs;
mod wiz_files;
mod wiz_texts;
mod wiz_trans;

pub type LizError = Box<dyn Error + Send + Sync>;

pub fn run(path: &str, args: Option<Vec<String>>) -> Result<Vec<String>, LizError> {
    let handler = rise(path, args)?;
    race(path, &handler)
}

pub fn rise(path: &str, args: Option<Vec<String>>) -> Result<Lua, LizError> {
    let handler = Lua::new();
    let mut error: Option<LizError> = None;
    handler.context(|ctx| {
        if let Err(err) = wiz_all::inject_all(ctx, path, args) {
            error = Some(err);
        }
    });
    if let Some(err) = error {
        return Err(err);
    }
    Ok(handler)
}

pub fn race(path: &str, handler: &Lua) -> Result<Vec<String>, LizError> {
    let mut result: Option<Result<Vec<String>, LizError>> = None;
    handler.context(|ctx| result = Some(race_in(ctx, path)));
    result
        .ok_or("Could not reach a result")
        .map_err(|err| utils::dbg_p("lib", "race", "ok_or", &[("path", path)], err))?
}

pub fn race_in(ctx: Context, path: &str) -> Result<Vec<String>, LizError> {
    let globals = ctx.globals();
    let liz: Table = globals.get("liz")?;

    let path = utils::add_liz_extension(path);
    let path = if liz_files::is_relative(&path) {
        let stack_dir = utils::get_stack_dir(&liz)?;
        liz_files::path_join(&stack_dir, &path)?
    } else {
        path
    };

    let race_pwd = liz_files::pwd()?;
    liz.set("race_pwd", race_pwd)?;

    let race_dir = liz_files::path_parent(&path)?;
    utils::put_stack_dir(&ctx, &liz, race_dir.clone())?;
    liz.set("race_dir", race_dir)?;

    let race_path = liz_files::path_absolute(&path)?;
    liz.set("race_path", race_path.clone())?;

    let source = std::fs::read_to_string(race_path)?;
    let mut source = source.trim();
    if source.starts_with("#!") {
        if let Some(first_line) = source.find("\n") {
            source = (&source[first_line + 1..]).trim();
        }
    }
    let values = ctx.load(source).eval::<MultiValue>()?;
    utils::pop_stack_dir(&liz)?;
    utils::to_json_multi(values)
}
