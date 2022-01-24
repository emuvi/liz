use rlua::{Lua, MultiValue, Table};
use simple_error::SimpleError;

use std::error::Error;
use std::path::Path;

pub mod codes;
pub mod execs;
pub mod files;
pub mod texts;
pub mod trans;
pub mod utils;

mod wizard;

pub type LizError = Box<dyn Error + Send + Sync>;

pub fn run(path: impl AsRef<Path>, args: Option<Vec<String>>) -> Result<Vec<String>, LizError> {
    let handler = rise(args)?;
    race(path, &handler)
}

pub fn race(path: impl AsRef<Path>, handler: &Lua) -> Result<Vec<String>, LizError> {
    let path = path.as_ref();
    let path_display = path
        .to_str()
        .ok_or("Could not get the display of the path to race.")?;
    if !path.exists() {
        let msg = format!("The path to load {} does not exists.", path_display);
        let err = SimpleError::new(msg);
        return Err(Box::new(err));
    }
    let parent = utils::load_parent(path)?;
    let parent_display = parent
        .to_str()
        .ok_or("Could not get the display of the parent to race.")?;
    let source = std::fs::read_to_string(&path)?;
    let mut result: Option<Result<Vec<String>, LizError>> = None;
    handler.context(|ctx| {
        let globals = ctx.globals();
        let liz: Option<Table> = match globals.get("liz") {
            Ok(liz) => Some(liz),
            Err(e) => {
                let msg = format!(
                    "Error on getting the liz global reference of {} with message: \n{}",
                    path_display, e
                );
                let err = SimpleError::new(msg);
                result = Some(Err(Box::new(err)));
                None
            }
        };
        if let Some(liz) = liz {
            if let Err(e) = liz.set("race_path", path_display) {
                let msg = format!(
                    "Error on setting the rece path on wizard of {} with message: \n{}",
                    path_display, e
                );
                let err = SimpleError::new(msg);
                result = Some(Err(Box::new(err)));
            }
            if let Err(e) = liz.set("race_dir", parent_display) {
                let msg = format!(
                    "Error on setting the race directory on wizard of {} with message: \n{}",
                    path_display, e
                );
                let err = SimpleError::new(msg);
                result = Some(Err(Box::new(err)));
            }
        }
    });
    if let Some(result) = result {
        return result;
    }
    handler.context(|ctx| match ctx.load(&source).eval::<MultiValue>() {
        Ok(values) => {
            let json_multi = utils::to_json_multi(values);
            match json_multi {
                Ok(json_multi) => result = Some(Ok(json_multi)),
                Err(e) => {
                    let mut msg = format!("Error on transforming the returned values of {} with message:\n", path_display);
                    let liz = utils::get_liz(ctx);
                    if let Some(liz) = liz {
                        if let Ok(err) = liz.get::<_, String>("err") {
                            msg.push_str(&err);
                        }
                    }
                    msg.push_str(&format!("{}", e));
                    let err = SimpleError::new(msg);
                    result = Some(Err(Box::new(err)));
                }
            }
        }
        Err(e) => {
            let mut msg = format!("Error on execution of {} with message:\n", path_display);
            let liz = utils::get_liz(ctx);
            if let Some(liz) = liz {
                if let Ok(err) = liz.get::<_, String>("err") {
                    msg.push_str(&err);
                }
            }
            msg.push_str(&format!("{}", e));
            let err = SimpleError::new(msg);
            result = Some(Err(Box::new(err)));
        }
    });
    match result {
        Some(result) => result,
        None => {
            let msg = format!("Could not reach a result on execution of {}", path_display);
            let err = SimpleError::new(msg);
            Err(Box::new(err))
        }
    }
}

pub fn rise(args: Option<Vec<String>>) -> Result<Lua, LizError> {
    let handler = Lua::new();
    let mut error: Option<LizError> = None;
    handler.context(|ctx| {
        if let Err(err) = wizard::inject_all(ctx, args) {
            error = Some(err);
        }
    });
    if let Some(err) = error {
        return Err(err);
    }
    Ok(handler)
}
