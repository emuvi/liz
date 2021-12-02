use rlua::{Context, Lua, MultiValue, Table};
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

pub mod tools;

pub type LizError = Box<dyn Error + Send + Sync>;

pub fn exec(path: impl AsRef<Path>, args: Option<Vec<String>>) -> Result<String, LizError> {
    let handler = start(args)?;
    load(path, &handler)
}

pub fn load(path: impl AsRef<Path>, handler: &Lua) -> Result<String, LizError> {
    let source = std::fs::read_to_string(&path)?;
    let path = path.as_ref();
    let path_display = path
        .to_str()
        .ok_or("Could not get the display of path to load.")?;
    let parent = parent(path)?;
    let old_dir = std::env::current_dir()?;
    let old_dir = if old_dir.is_relative() {
        std::fs::canonicalize(old_dir)?
    } else {
        old_dir
    };
    std::env::set_current_dir(parent)?;
    let mut result: Result<String, LizError> = Ok(String::default());
    handler.context(|ctx| {
        let globals = ctx.globals();
        let wiz: Option<Table> = match globals.get("wiz") {
            Ok(wiz) => {
                Some(wiz)
            },
            Err(e) => {
                let msg = format!(
                    "Error on getting the wizard of {} with message: \n{}",
                    path_display, e
                );
                let err = simple_error::SimpleError::new(msg);
                result = Err(Box::new(err));
                None
            },
        };
        if let Some(wiz) = wiz {
            if let Err(e) = wiz.set("path", path_display) {
                let msg = format!(
                    "Error on setting the path on wizard of {} with message: \n{}",
                    path_display, e
                );
                let err = simple_error::SimpleError::new(msg);
                result = Err(Box::new(err));
            }
        }
    });
    if result.is_err() {
        return result;
    }
    handler.context(|ctx| match ctx.load(&source).eval::<MultiValue>() {
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
                    "Successfully executed {} with result(s):\n{}",
                    path_display, returned
                )
            };
            result = Ok(msg);
        }
        Err(e) => {
            let mut msg = format!("Error on execution of {} with message:\n", path_display);
            let wiz = get_wiz(ctx);
            if let Some(wiz) = wiz {
                if let Ok(err) = wiz.get::<_, String>("err") {
                    msg.push_str(&err);
                }
            }
            msg.push_str(&format!("{}", e));
            let err = simple_error::SimpleError::new(msg);
            result = Err(Box::new(err));
        }
    });
    if let Err(e) = std::env::set_current_dir(old_dir) {
        let msg = format!(
            "Error on returning the previous current dir on execution of {} with message: \n{}",
            path_display, e
        );
        let err = simple_error::SimpleError::new(msg);
        result = Err(Box::new(err));
    }
    result
}

pub fn start(args: Option<Vec<String>>) -> Result<Lua, LizError> {
    let handler = Lua::new();
    let mut error: Option<LizError> = None;
    handler.context(|ctx| {
        if let Err(err) = wiz_injection(ctx, args) {
            error = Some(err);
        }
    });
    if let Some(err) = error {
        return Err(err);
    }
    Ok(handler)
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
        .ok_or("Could not get the parent from the path.")?;
    Ok(PathBuf::from(parent))
}

fn get_wiz(from_ctx: Context) -> Option<Table> {
    let globals = from_ctx.globals();
    match globals.get("wiz") {
        Ok(wiz) => Some(wiz),
        Err(_) => None,
    }
}

fn treat_error<T>(ctx: Context, result: Result<T, LizError>) -> Result<T, rlua::Error> {
    match result {
        Ok(returned) => Ok(returned),
        Err(error) => {
            if let Some(wiz) = get_wiz(ctx) {
                let mut new = true;
                if let Ok(has) = wiz.contains_key("err") {
                    new = !has;
                }
                let mut stack_err: String = if !new {
                    match wiz.get("err") {
                        Ok(old_stacked) => old_stacked,
                        Err(get_old_err) => {
                            eprintln!("Could not get the stacked errors because: {}", get_old_err);
                            String::new()
                        }
                    }
                } else {
                    String::new()
                };
                stack_err.push_str(&format!("{}\n", error));
                if let Err(not_set_err) = wiz.set("err", stack_err) {
                    eprintln!("Could not set the error stack because: {}", not_set_err);
                }
            } else {
                eprintln!("Could not set the error stack because: Could not get the wiz.",);
            }
            Err(rlua::Error::external(error))
        }
    }
}

fn wiz_injection(ctx: Context, args: Option<Vec<String>>) -> Result<(), LizError> {
    let wiz = ctx.create_table()?;

    wiz.set("args", args)?;

    let cmd = ctx.create_function(
        |ctx, (name, args, dir, print, throw): (String, Vec<String>, String, bool, bool)| {
            treat_error(ctx, tools::cmd(&name, args.as_slice(), &dir, print, throw))
        },
    )?;
    wiz.set("cmd", cmd)?;

    let has = ctx.create_function(|_, path: String| Ok(tools::has(&path)))?;
    wiz.set("has", has)?;

    let is_dir = ctx.create_function(|_, path: String| Ok(tools::is_dir(&path)))?;
    wiz.set("is_dir", is_dir)?;

    let is_file = ctx.create_function(|_, path: String| Ok(tools::is_file(&path)))?;
    wiz.set("is_file", is_file)?;

    let cd = ctx.create_function(|ctx, path: String| treat_error(ctx, tools::cd(&path)))?;
    wiz.set("cd", cd)?;

    let rn = ctx.create_function(|ctx, (origin, destiny): (String, String)| {
        treat_error(ctx, tools::rn(&origin, &destiny))
    })?;
    wiz.set("rn", rn)?;

    let cp = ctx.create_function(|ctx, (origin, destiny): (String, String)| {
        treat_error(ctx, tools::cp(&origin, &destiny))
    })?;
    wiz.set("cp", cp)?;

    let cp_tmp = ctx.create_function(|ctx, (origin, destiny): (String, String)| {
        treat_error(ctx, tools::cp_tmp(&origin, &destiny))
    })?;
    wiz.set("cp_tmp", cp_tmp)?;

    let mv = ctx.create_function(|ctx, (origin, destiny): (String, String)| {
        treat_error(ctx, tools::mv(&origin, &destiny))
    })?;
    wiz.set("mv", mv)?;

    let rm = ctx.create_function(|ctx, path: String| treat_error(ctx, tools::rm(&path)))?;
    wiz.set("rm", rm)?;

    let read = ctx.create_function(|ctx, path: String| treat_error(ctx, tools::read(&path)))?;
    wiz.set("read", read)?;

    let mk_dir = ctx.create_function(|ctx, path: String| treat_error(ctx, tools::mk_dir(&path)))?;
    wiz.set("mk_dir", mk_dir)?;

    let touch = ctx.create_function(|ctx, path: String| treat_error(ctx, tools::touch(&path)))?;
    wiz.set("touch", touch)?;

    let write = ctx.create_function(|ctx, (path, contents): (String, String)| {
        treat_error(ctx, tools::write(&path, &contents))
    })?;
    wiz.set("write", write)?;

    let append = ctx.create_function(|ctx, (path, contents): (String, String)| {
        treat_error(ctx, tools::append(&path, &contents))
    })?;
    wiz.set("append", append)?;

    wiz.set("exe_ext", tools::exe_ext())?;

    let path_ext =
        ctx.create_function(|ctx, path: String| treat_error(ctx, tools::path_ext(&path)))?;
    wiz.set("path_ext", path_ext)?;

    let path_name =
        ctx.create_function(|ctx, path: String| treat_error(ctx, tools::path_name(&path)))?;
    wiz.set("path_name", path_name)?;

    let path_steam =
        ctx.create_function(|ctx, path: String| treat_error(ctx, tools::path_steam(&path)))?;
    wiz.set("path_steam", path_steam)?;

    let path_parent =
        ctx.create_function(|ctx, path: String| treat_error(ctx, tools::path_parent(&path)))?;
    wiz.set("path_parent", path_parent)?;

    let path_join = ctx.create_function(|ctx, (path, child): (String, String)| {
        treat_error(ctx, tools::path_join(&path, &child))
    })?;
    wiz.set("path_join", path_join)?;

    let globals = ctx.globals();
    globals.set("wiz", wiz)?;
    Ok(())
}
