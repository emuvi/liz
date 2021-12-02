use rlua::{Context, Lua, MultiValue};
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

pub mod tools;

pub type LizError = Box<dyn Error + Send + Sync>;

pub fn execute(path: impl AsRef<Path>
) -> Result<String, LizError> {
    let source = std::fs::read_to_string(&path)?;
    let path = path.as_ref();
    let path_display = path
        .to_str()
        .ok_or("Could not get the display of path to execute.")?;
    let parent = parent(path)?;
    let old_dir = std::env::current_dir()?;
    let old_dir = if old_dir.is_relative() {
        std::fs::canonicalize(old_dir)?
    } else {
        old_dir
    };
    std::env::set_current_dir(parent)?;
    let lua = Lua::new();
    let mut result: Result<String, LizError> = Ok(String::default());
    lua.context(|ctx| {
        if let Err(e) = wiz_injection(ctx) {
            let msg = format!(
                "Error in injection on execution of {} with message: \n{}",
                path_display, e
            );
            let err = simple_error::SimpleError::new(msg);
            result = Err(Box::new(err));
        }
    });
    if result.is_err() {
        return result;
    }
    lua.context(|ctx| match ctx.load(&source).eval::<MultiValue>() {
        Ok(values) => {
            let returned = format!(
                "{}",
                values
                    .iter()
                    .map(|value| format!("{:?}", value))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            let returned = returned.trim();
            let msg = if returned.is_empty() {
                format!("Successfully executed {} with no result.", path_display)
            } else {
                format!(
                    "Successfully executed {} with result(s): \n{}",
                    path_display, returned
                )
            };
            result = Ok(msg);
        }
        Err(e) => {
            let msg = format!(
                "Error on execution of {} with message: \n{}",
                path_display, e
            );
            let err = simple_error::SimpleError::new(msg);
            result = Err(Box::new(err));
        }
    });
    if let Err(error) = std::env::set_current_dir(old_dir) {
        panic!(
            "Could not return to the previous working directory. - {}",
            error
        );
    }
    result
}

fn parent(path: impl AsRef<Path>) -> Result<PathBuf, LizError> {
    let path = path.as_ref();
    let path = if path.is_relative() {
        std::fs::canonicalize(path)?
    } else {
        path.to_path_buf()
    };
    let parent = path
        .parent()
        .ok_or("Could not get the parent from the path")?;
    Ok(std::path::PathBuf::from(parent))
}

fn to_lua<T>(result: Result<T, LizError>) -> Result<T, rlua::Error> {
    match result {
        Ok(returned) => Ok(returned),
        Err(error) => {
            let error = format!("{}", error);
            Err(rlua::Error::RuntimeError(error))
        },
    }
}

fn wiz_injection(ctx: Context) -> Result<(), LizError> {
    let wiz = ctx.create_table()?;

    let cmd = ctx.create_function(
        |_, (name, args, dir, print, throw): (String, Vec<String>, String, bool, bool)| {
            to_lua(tools::cmd(&name, args.as_slice(), &dir, print, throw))
        },
    )?;
    wiz.set("cmd", cmd)?;

    let has = ctx.create_function(|_, path: String| Ok(tools::has(&path)))?;
    wiz.set("has", has)?;

    let is_dir = ctx.create_function(|_, path: String| Ok(tools::is_dir(&path)))?;
    wiz.set("is_dir", is_dir)?;

    let is_file = ctx.create_function(|_, path: String| Ok(tools::is_file(&path)))?;
    wiz.set("is_file", is_file)?;

    let cd = ctx.create_function(|_, path: String| {
        to_lua(tools::cd(&path))
    })?;
    wiz.set("cd", cd)?;

    let rn = ctx.create_function(|_, (origin, destiny): (String, String)| {
        to_lua(tools::rn(&origin, &destiny))
    })?;
    wiz.set("rn", rn)?;

    let cp = ctx.create_function(|_, (origin, destiny): (String, String)| {
        to_lua(tools::cp(&origin, &destiny))
    })?;
    wiz.set("cp", cp)?;

    let cp_tmp = ctx.create_function(|_, (origin, destiny): (String, String)| {
        to_lua(tools::cp_tmp(&origin, &destiny))
    })?;
    wiz.set("cp_tmp", cp_tmp)?;

    let mv = ctx.create_function(|_, (origin, destiny): (String, String)| {
        to_lua(tools::mv(&origin, &destiny))
    })?;
    wiz.set("mv", mv)?;

    let rm = ctx.create_function(|_, path: String| to_lua(tools::rm(&path)))?;
    wiz.set("rm", rm)?;

    let read = ctx.create_function(|_, path: String| to_lua(tools::read(&path)))?;
    wiz.set("read", read)?;

    let mk_dir = ctx.create_function(|_, path: String| to_lua(tools::mk_dir(&path)))?;
    wiz.set("mk_dir", mk_dir)?;

    let touch = ctx.create_function(|_, path: String| to_lua(tools::touch(&path)))?;
    wiz.set("touch", touch)?;

    let write = ctx.create_function(|_, (path, contents): (String, String)| {
        to_lua(tools::write(&path, &contents))
    })?;
    wiz.set("write", write)?;

    let append = ctx.create_function(|_, (path, contents): (String, String)| {
        to_lua(tools::append(&path, &contents))
    })?;
    wiz.set("append", append)?;

    wiz.set("exe_ext", tools::exe_ext())?;

    let globals = ctx.globals();
    globals.set("wiz", wiz)?;
    Ok(())
}
