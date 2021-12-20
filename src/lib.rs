use rlua::{Context, Lua, MultiValue, Table, Value};
use simple_error::SimpleError;
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

pub mod codes;
pub mod execs;
pub mod files;
pub mod texts;

use execs::Spawned;

pub type LizError = Box<dyn Error + Send + Sync>;

pub fn exec(path: impl AsRef<Path>, args: Option<Vec<String>>) -> Result<Vec<String>, LizError> {
    let handler = start(args)?;
    load(path, &handler)
}

pub fn load(path: impl AsRef<Path>, handler: &Lua) -> Result<Vec<String>, LizError> {
    let path = path.as_ref();
    let path_display = path
        .to_str()
        .ok_or("Could not get the display of the path to load.")?;
    if !path.exists() {
        let msg = format!("The path to load {} does not exists.", path_display);
        let err = SimpleError::new(msg);
        return Err(Box::new(err));
    }
    let source = std::fs::read_to_string(&path)?;
    let parent = load_parent(path)?;
    let old_dir = std::env::current_dir()?;
    let old_dir = if old_dir.is_relative() {
        std::fs::canonicalize(old_dir)?
    } else {
        old_dir
    };
    std::env::set_current_dir(parent)?;
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
            if let Err(e) = liz.set("path", path_display) {
                let msg = format!(
                    "Error on setting the path on wizard of {} with message: \n{}",
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
            let mut vector: Vec<String> = Vec::new();
            for value in &values {
                let value_str = match value {
                    Value::Nil => format!("|Nil|[]"),
                    Value::Boolean(data) => format!("|Boolean|[{}]", data),
                    Value::Integer(data) => format!("|Integer|[{}]", data),
                    Value::Number(data) => format!("|Number|[{}]", data),
                    Value::String(data) => match data.to_str() {
                        Ok(data) => {
                            format!("|String|[{}]", data)
                        }
                        Err(error) => {
                            format!("|String|[![{}]]", error)
                        }
                    },
                    Value::Table(data) => format!("|Table|[{:?}]", data),
                    Value::Function(data) => format!("|Function|[{:?}]", data),
                    Value::LightUserData(data) => format!("|LightUserData|[{:?}]", data),
                    Value::UserData(data) => format!("|UserData|[{:?}]", data),
                    Value::Thread(data) => format!("|Thread|[{:?}]", data),
                    Value::Error(data) => format!("|Error|[{:?}]", data),
                };
                vector.push(value_str);
            }
            result = Some(Ok(vector));
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
            let err = SimpleError::new(msg);
            result = Some(Err(Box::new(err)));
        }
    });
    if let Err(e) = std::env::set_current_dir(old_dir) {
        let msg = format!(
            "Error on returning the previous current dir on execution of {} with message: \n{}",
            path_display, e
        );
        let err = SimpleError::new(msg);
        result = Some(Err(Box::new(err)));
    }
    match result {
        Some(result) => result,
        None => {
            let msg = format!("Could not reach a result on execution of {}", path_display);
            let err = SimpleError::new(msg);
            Err(Box::new(err))
        }
    }
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

fn load_parent(path: impl AsRef<Path>) -> Result<PathBuf, LizError> {
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

fn from_returned<'a>(ctx: Context<'a>, value: String) -> Result<Value<'a>, LizError> {
    if value.starts_with("|Nil|") {
        return Ok(Value::Nil);
    } else if value.starts_with("|Boolean|") {
        if value == "|Boolean|[true]" {
            return Ok(Value::Boolean(true));
        } else {
            return Ok(Value::Boolean(false));
        }
    } else if value.starts_with("|Integer|") {
        let data = &value[10..value.len() - 1];
        let data: i64 = data.parse()?;
        return Ok(Value::Integer(data));
    } else if value.starts_with("|Number|") {
        let data = &value[9..value.len() - 1];
        let data: f64 = data.parse()?;
        return Ok(Value::Number(data));
    } else if value.starts_with("|String|") {
        let data = &value[9..value.len() - 1];
        let data = ctx.create_string(data)?;
        return Ok(Value::String(data));
    }
    let data = ctx.create_string(&value)?;
    return Ok(Value::String(data));
}

fn liz_injection(ctx: Context, args: Option<Vec<String>>) -> Result<(), LizError> {
    let liz = ctx.create_table()?;
    liz.set("args", args)?;
    let from_returned = ctx
        .create_function(|ctx, returned: String| treat_error(ctx, from_returned(ctx, returned)))?;
    liz.set("from_returned", from_returned)?;
    liz_inject_codes(ctx, &liz)?;
    liz_inject_execs(ctx, &liz)?;
    liz_inject_files(ctx, &liz)?;
    liz_inject_texts(ctx, &liz)?;
    let globals = ctx.globals();
    globals.set("liz", liz)?;
    Ok(())
}

fn liz_inject_codes<'a>(ctx: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let git_root_find = ctx.create_function(|ctx, path: String| {
        treat_error(ctx, codes::git_root_find(&path))
    })?;
    liz.set("git_root_find", git_root_find)?;

	let git_is_ignored = ctx.create_function(|ctx, path: String| {
        treat_error(ctx, codes::git_is_ignored(&path))
    })?;
    liz.set("git_is_ignored", git_is_ignored)?;

	Ok(())
}

fn liz_inject_execs<'a>(ctx: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let exec = ctx.create_function(|ctx, (path, args): (String, Option<Vec<String>>)| {
        treat_error(ctx, exec(path, args))
    })?;
    liz.set("exec", exec)?;

    let spawn = ctx.create_function(|_, (path, args): (String, Option<Vec<String>>)| {
        Ok(execs::spawn(path, args))
    })?;
    liz.set("spawn", spawn)?;

    let join =
        ctx.create_function(|ctx, spawned: Spawned| treat_error(ctx, execs::join(spawned)))?;
    liz.set("join", join)?;

    let cmd = ctx.create_function(
        |ctx, (name, args, dir, print, throw): (String, Vec<String>, String, bool, bool)| {
            treat_error(ctx, execs::cmd(&name, args.as_slice(), &dir, print, throw))
        },
    )?;
    liz.set("cmd", cmd)?;

    Ok(())
}

fn liz_inject_files<'a>(ctx: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let has = ctx.create_function(|_, path: String| Ok(files::has(&path)))?;
    liz.set("has", has)?;

    let is_dir = ctx.create_function(|_, path: String| Ok(files::is_dir(&path)))?;
    liz.set("is_dir", is_dir)?;

    let is_file = ctx.create_function(|_, path: String| Ok(files::is_file(&path)))?;
    liz.set("is_file", is_file)?;

    let cd = ctx.create_function(|ctx, path: String| treat_error(ctx, files::cd(&path)))?;
    liz.set("cd", cd)?;

    let pwd = ctx.create_function(|ctx, ()| treat_error(ctx, files::pwd()))?;
    liz.set("pwd", pwd)?;

    let rn = ctx.create_function(|ctx, (origin, destiny): (String, String)| {
        treat_error(ctx, files::rn(&origin, &destiny))
    })?;
    liz.set("rn", rn)?;

    let cp = ctx.create_function(|ctx, (origin, destiny): (String, String)| {
        treat_error(ctx, files::cp(&origin, &destiny))
    })?;
    liz.set("cp", cp)?;

    let cp_tmp = ctx.create_function(|ctx, (origin, destiny): (String, String)| {
        treat_error(ctx, files::cp_tmp(&origin, &destiny))
    })?;
    liz.set("cp_tmp", cp_tmp)?;

    let mv = ctx.create_function(|ctx, (origin, destiny): (String, String)| {
        treat_error(ctx, files::mv(&origin, &destiny))
    })?;
    liz.set("mv", mv)?;

    let rm = ctx.create_function(|ctx, path: String| treat_error(ctx, files::rm(&path)))?;
    liz.set("rm", rm)?;

    let read = ctx.create_function(|ctx, path: String| treat_error(ctx, files::read(&path)))?;
    liz.set("read", read)?;

    let mk_dir = ctx.create_function(|ctx, path: String| treat_error(ctx, files::mk_dir(&path)))?;
    liz.set("mk_dir", mk_dir)?;

    let touch = ctx.create_function(|ctx, path: String| treat_error(ctx, files::touch(&path)))?;
    liz.set("touch", touch)?;

    let write = ctx.create_function(|ctx, (path, contents): (String, String)| {
        treat_error(ctx, files::write(&path, &contents))
    })?;
    liz.set("write", write)?;

    let append = ctx.create_function(|ctx, (path, contents): (String, String)| {
        treat_error(ctx, files::append(&path, &contents))
    })?;
    liz.set("append", append)?;

    liz.set("exe_ext", files::exe_ext())?;

    liz.set("path_sep", files::path_sep())?;

    let path_ext =
        ctx.create_function(|ctx, path: String| treat_error(ctx, files::path_ext(&path)))?;
    liz.set("path_ext", path_ext)?;

    let path_name =
        ctx.create_function(|ctx, path: String| treat_error(ctx, files::path_name(&path)))?;
    liz.set("path_name", path_name)?;

    let path_stem =
        ctx.create_function(|ctx, path: String| treat_error(ctx, files::path_stem(&path)))?;
    liz.set("path_stem", path_stem)?;

	let path_absolute = ctx.create_function(|ctx, path: String| {
        treat_error(ctx, files::path_absolute(&path))
    })?;
    liz.set("path_absolute", path_absolute)?;

	let path_relative = ctx.create_function(|ctx, (path, base): (String, String)| {
        treat_error(ctx, files::path_relative(&path, &base))
    })?;
    liz.set("path_relative", path_relative)?;

    let path_parent =
        ctx.create_function(|ctx, path: String| treat_error(ctx, files::path_parent(&path)))?;
    liz.set("path_parent", path_parent)?;

    let path_parent_find = ctx.create_function(|ctx, (path, with_name): (String, String)| {
        treat_error(ctx, files::path_parent_find(&path, &with_name))
    })?;
    liz.set("path_parent_find", path_parent_find)?;

    let path_join = ctx.create_function(|ctx, (path, child): (String, String)| {
        treat_error(ctx, files::path_join(&path, &child))
    })?;
    liz.set("path_join", path_join)?;

    let path_list =
        ctx.create_function(|ctx, path: String| treat_error(ctx, files::path_list(&path)))?;
    liz.set("path_list", path_list)?;

	let path_list_subs =
        ctx.create_function(|ctx, path: String| treat_error(ctx, files::path_list_subs(&path)))?;
    liz.set("path_list_subs", path_list_subs)?;

    let path_list_dirs =
        ctx.create_function(|ctx, path: String| treat_error(ctx, files::path_list_dirs(&path)))?;
    liz.set("path_list_dirs", path_list_dirs)?;

    let path_list_dirs_subs =
        ctx.create_function(|ctx, path: String| treat_error(ctx, files::path_list_dirs_subs(&path)))?;
    liz.set("path_list_dirs_subs", path_list_dirs_subs)?;

    let path_list_files =
        ctx.create_function(|ctx, path: String| treat_error(ctx, files::path_list_files(&path)))?;
    liz.set("path_list_files", path_list_files)?;

    let path_list_files_subs =
        ctx.create_function(|ctx, path: String| treat_error(ctx, files::path_list_files_subs(&path)))?;
    liz.set("path_list_files_subs", path_list_files_subs)?;

    let path_list_files_ext = ctx.create_function(|ctx, (path, ext): (String, String)| {
        treat_error(ctx, files::path_list_files_ext(&path, &ext))
    })?;
    liz.set("path_list_files_ext", path_list_files_ext)?;
	
    let path_list_files_exts = ctx.create_function(|ctx, (path, exts): (String, Vec<String>)| {
		let exts: Vec<_> = exts.iter().map(String::as_ref).collect();
        treat_error(ctx, files::path_list_files_exts(&path, &exts))
    })?;
    liz.set("path_list_files_exts", path_list_files_exts)?;
	
	let path_list_files_ext_subs = ctx.create_function(|ctx, (path, ext): (String, String)| {
        treat_error(ctx, files::path_list_files_ext_subs(&path, &ext))
    })?;
    liz.set("path_list_files_ext_subs", path_list_files_ext_subs)?;

	let path_list_files_exts_subs = ctx.create_function(|ctx, (path, exts): (String, Vec<String>)| {
		let exts: Vec<_> = exts.iter().map(String::as_str).collect();
        treat_error(ctx, files::path_list_files_exts_subs(&path, &exts))
    })?;
    liz.set("path_list_files_exts_subs", path_list_files_exts_subs)?;

    Ok(())
}

fn liz_inject_texts<'a>(ctx: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
	let text_trim = ctx.create_function(
        |_, text: String| {
            Ok(texts::text_trim(&text))
        },
    )?;
    liz.set("text_trim", text_trim)?;


	let text_path_find = ctx.create_function(
        |ctx, (path, contents): (String, String)| {
            treat_error(ctx, texts::text_path_find(&path, &contents))
        },
    )?;
    liz.set("text_path_find", text_path_find)?;

	let text_dir_find = ctx.create_function(
        |ctx, (path, contents): (String, String)| {
            treat_error(ctx, texts::text_dir_find(&path, &contents))
        },
    )?;
    liz.set("text_dir_find", text_dir_find)?;

	let text_file_find = ctx.create_function(
        |ctx, (path, contents): (String, String)| {
            treat_error(ctx, texts::text_file_find(&path, &contents))
        },
    )?;
    liz.set("text_file_find", text_file_find)?;

    Ok(())
}
