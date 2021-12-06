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
    let path = path.as_ref();
    let path_display = path
        .to_str()
        .ok_or("Could not get the display of the path to load.")?;
    if !path.exists() {
        let msg = format!("The path to load {} does not exists.", path_display);
        let err = simple_error::SimpleError::new(msg);
        return Err(Box::new(err));
    }
    let source = std::fs::read_to_string(&path)?;
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
        let liz: Option<Table> = match globals.get("liz") {
            Ok(liz) => Some(liz),
            Err(e) => {
                let msg = format!(
                    "Error on getting the liz global reference of {} with message: \n{}",
                    path_display, e
                );
                let err = simple_error::SimpleError::new(msg);
                result = Err(Box::new(err));
                None
            }
        };
        if let Some(liz) = liz {
            if let Err(e) = liz.set("path", path_display) {
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
            let liz = get_liz(ctx);
            if let Some(liz) = liz {
                if let Ok(err) = liz.get::<_, String>("err") {
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
        if let Err(err) = liz_injection(ctx, args) {
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

fn get_liz(from_ctx: Context) -> Option<Table> {
    let globals = from_ctx.globals();
    match globals.get("liz") {
        Ok(liz) => Some(liz),
        Err(_) => None,
    }
}

fn treat_error<T>(ctx: Context, result: Result<T, LizError>) -> Result<T, rlua::Error> {
    match result {
        Ok(returned) => Ok(returned),
        Err(error) => {
            if let Some(liz) = get_liz(ctx) {
                let mut new = true;
                if let Ok(has) = liz.contains_key("err") {
                    new = !has;
                }
                let mut stack_err: String = if !new {
                    match liz.get("err") {
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
                if let Err(not_set_err) = liz.set("err", stack_err) {
                    eprintln!("Could not set the error stack because: {}", not_set_err);
                }
            } else {
                eprintln!("Could not set the error stack because: Could not get the liz.",);
            }
            Err(rlua::Error::external(error))
        }
    }
}

fn liz_injection(ctx: Context, args: Option<Vec<String>>) -> Result<(), LizError> {
    let liz = ctx.create_table()?;

    liz.set("args", args)?;

    let cmd = ctx.create_function(
        |ctx, (name, args, dir, print, throw): (String, Vec<String>, String, bool, bool)| {
            treat_error(ctx, tools::cmd(&name, args.as_slice(), &dir, print, throw))
        },
    )?;
    liz.set("cmd", cmd)?;

    let has = ctx.create_function(|_, path: String| Ok(tools::has(&path)))?;
    liz.set("has", has)?;

    let is_dir = ctx.create_function(|_, path: String| Ok(tools::is_dir(&path)))?;
    liz.set("is_dir", is_dir)?;

    let is_file = ctx.create_function(|_, path: String| Ok(tools::is_file(&path)))?;
    liz.set("is_file", is_file)?;

    let cd = ctx.create_function(|ctx, path: String| treat_error(ctx, tools::cd(&path)))?;
    liz.set("cd", cd)?;

    let pwd = ctx.create_function(|ctx, ()| treat_error(ctx, tools::pwd()))?;
    liz.set("pwd", pwd)?;

    let rn = ctx.create_function(|ctx, (origin, destiny): (String, String)| {
        treat_error(ctx, tools::rn(&origin, &destiny))
    })?;
    liz.set("rn", rn)?;

    let cp = ctx.create_function(|ctx, (origin, destiny): (String, String)| {
        treat_error(ctx, tools::cp(&origin, &destiny))
    })?;
    liz.set("cp", cp)?;

    let cp_tmp = ctx.create_function(|ctx, (origin, destiny): (String, String)| {
        treat_error(ctx, tools::cp_tmp(&origin, &destiny))
    })?;
    liz.set("cp_tmp", cp_tmp)?;

    let mv = ctx.create_function(|ctx, (origin, destiny): (String, String)| {
        treat_error(ctx, tools::mv(&origin, &destiny))
    })?;
    liz.set("mv", mv)?;

    let rm = ctx.create_function(|ctx, path: String| treat_error(ctx, tools::rm(&path)))?;
    liz.set("rm", rm)?;

    let read = ctx.create_function(|ctx, path: String| treat_error(ctx, tools::read(&path)))?;
    liz.set("read", read)?;

    let mk_dir = ctx.create_function(|ctx, path: String| treat_error(ctx, tools::mk_dir(&path)))?;
    liz.set("mk_dir", mk_dir)?;

    let touch = ctx.create_function(|ctx, path: String| treat_error(ctx, tools::touch(&path)))?;
    liz.set("touch", touch)?;

    let write = ctx.create_function(|ctx, (path, contents): (String, String)| {
        treat_error(ctx, tools::write(&path, &contents))
    })?;
    liz.set("write", write)?;

    let append = ctx.create_function(|ctx, (path, contents): (String, String)| {
        treat_error(ctx, tools::append(&path, &contents))
    })?;
    liz.set("append", append)?;

    liz.set("exe_ext", tools::exe_ext())?;

    liz.set("path_sep", tools::path_sep())?;

    let path_ext =
        ctx.create_function(|ctx, path: String| treat_error(ctx, tools::path_ext(&path)))?;
    liz.set("path_ext", path_ext)?;

    let path_name =
        ctx.create_function(|ctx, path: String| treat_error(ctx, tools::path_name(&path)))?;
    liz.set("path_name", path_name)?;

    let path_stem =
        ctx.create_function(|ctx, path: String| treat_error(ctx, tools::path_stem(&path)))?;
    liz.set("path_stem", path_stem)?;

    let path_parent =
        ctx.create_function(|ctx, path: String| treat_error(ctx, tools::path_parent(&path)))?;
    liz.set("path_parent", path_parent)?;

    let path_parent_find =
        ctx.create_function(|ctx, (path, with_name): (String, String)| treat_error(ctx, tools::path_parent_find(&path, &with_name)))?;
    liz.set("path_parent_find", path_parent_find)?;

    let path_join = ctx.create_function(|ctx, (path, child): (String, String)| {
        treat_error(ctx, tools::path_join(&path, &child))
    })?;
    liz.set("path_join", path_join)?;

    let path_list = ctx.create_function(|ctx, path: String| {
        treat_error(ctx, tools::path_list(&path))
    })?;
    liz.set("path_list", path_list)?;

    let path_list_dirs = ctx.create_function(|ctx, path: String| {
        treat_error(ctx, tools::path_list_dirs(&path))
    })?;
    liz.set("path_list_dirs", path_list_dirs)?;

    let path_list_files = ctx.create_function(|ctx, path: String| {
        treat_error(ctx, tools::path_list_files(&path))
    })?;
    liz.set("path_list_files", path_list_files)?;

    let globals = ctx.globals();
    globals.set("liz", liz)?;
    Ok(())
}
