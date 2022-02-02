use rlua::{Context, Lua, MultiValue, Table};
use simple_error::SimpleError;

use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

pub mod liz_codes;
pub mod liz_execs;
pub mod liz_files;
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

pub fn run(path: impl AsRef<Path>, args: Option<Vec<String>>) -> Result<Vec<String>, LizError> {
    let handler = rise(&path, args)?;
    race(path, &handler)
}

pub fn rise(path: impl AsRef<Path>, args: Option<Vec<String>>) -> Result<Lua, LizError> {
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

pub fn race(path: impl AsRef<Path>, handler: &Lua) -> Result<Vec<String>, LizError> {
    let mut result: Option<Result<Vec<String>, LizError>> = None;
    handler.context(|context| result = Some(race_in(path, context)));
    match result {
        Some(result) => result,
        None => {
            let msg = format!("Could not reach a result on the execution.");
            let err = SimpleError::new(msg);
            Err(Box::new(err))
        }
    }
}

pub fn race_in(path: impl AsRef<Path>, context: Context) -> Result<Vec<String>, LizError> {
    let globals = context.globals();
    let liz: Table = globals.get("liz")?;

    let path = path.as_ref();
    let path = if path.is_relative() {
        let rise_dir: String = liz.get("rise_dir")?;
        let rise_dir: PathBuf = rise_dir.into();
        rise_dir.join(path)
    } else {
        path.into()
    };

    let race_pwd = liz_files::pwd()?;
    liz.set("race_pwd", race_pwd)?;

    let race_dir = liz_files::path_parent(&path)?;
    liz.set("race_dir", race_dir)?;

    let mut race_path = liz_files::path_absolute(path)?;
    let race_path_check = race_path.to_lowercase();
    if ! (race_path_check.ends_with(".liz") || race_path_check.ends_with(".lua")) {
        race_path.push_str(".liz");
    }
    liz.set("race_path", race_path.clone())?;

    let source = std::fs::read_to_string(race_path)?;
    let mut source = source.trim();
    if source.starts_with("#!") {
        if let Some(first_line) = source.find("\n") {
            source = (&source[first_line + 1..]).trim();
        }
    }
    let values = context.load(source).eval::<MultiValue>()?;
    utils::to_json_multi(values)
}
