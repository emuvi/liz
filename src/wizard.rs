use rlua::{Context, Table, Value, MultiValue};

use std::collections::HashMap;

use crate::codes;
use crate::execs;
use crate::files;
use crate::texts;
use crate::trans;
use crate::utils;

// TODO - Separate this big file into smaller ones for each module.

use crate::execs::Spawned;
use crate::LizError;

pub fn inject_all(ctx: Context, args: Option<Vec<String>>) -> Result<(), LizError> {
    let liz = ctx.create_table()?;
    liz.set("args", args)?;

    let path = std::env::current_dir()?;
    let path_display = path
        .to_str()
        .ok_or("Could not get the display path of the rise.")?;
    liz.set("rise_dir", String::from(path_display))?;

    let to_json_multi = ctx.create_function(|ctx, values: MultiValue| {
        utils::treat_error(ctx, utils::to_json_multi(values))
    })?;

    let to_json = ctx.create_function(|ctx, value: Value| {
        utils::treat_error(ctx, utils::to_json(value))
    })?;

    let from_json = ctx.create_function(|ctx, source: String| {
        utils::treat_error(ctx, utils::from_json(ctx, source))
    })?;
    
    liz.set("to_json_multi", to_json_multi)?;
    liz.set("to_json", to_json)?;
    liz.set("from_json", from_json)?;

    inject_codes(ctx, &liz)?;
    inject_execs(ctx, &liz)?;
    inject_files(ctx, &liz)?;
    inject_texts(ctx, &liz)?;
    inject_trans(ctx, &liz)?;

    let globals = ctx.globals();
    globals.set("liz", liz)?;

    Ok(())
}

fn inject_codes<'a>(ctx: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let git_root_find = ctx.create_function(|ctx, path: String| {
        utils::treat_error(ctx, codes::git_root_find(&path))
    })?;

    let git_is_ignored = ctx.create_function(|ctx, path: String| {
        utils::treat_error(ctx, codes::git_is_ignored(&path))
    })?;

    liz.set("git_root_find", git_root_find)?;
    liz.set("git_is_ignored", git_is_ignored)?;

    Ok(())
}

fn inject_execs<'a>(ctx: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let run = ctx.create_function(|ctx, (path, args): (String, Option<Vec<String>>)| {
        utils::treat_error(ctx, crate::run(path, args))
    })?;

    let spawn = ctx.create_function(|_, (path, args): (String, Option<Vec<String>>)| {
        Ok(execs::spawn(path, args))
    })?;

    let join =
        ctx.create_function(|ctx, spawned: Spawned| utils::treat_error(ctx, execs::join(spawned)))?;

    let cmd = ctx.create_function(
        |ctx, (name, args, dir, print, throw): (String, Vec<String>, String, bool, bool)| {
            utils::treat_error(ctx, execs::cmd(&name, args.as_slice(), &dir, print, throw))
        },
    )?;

    let pause = ctx.create_function(|_, ()| Ok(execs::pause()))?;

    liz.set("run", run)?;
    liz.set("spawn", spawn)?;
    liz.set("join", join)?;
    liz.set("cmd", cmd)?;
    liz.set("pause", pause)?;

    Ok(())
}

fn inject_files<'a>(ctx: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let has = ctx.create_function(|_, path: String| Ok(files::has(&path)))?;

    let is_dir = ctx.create_function(|_, path: String| Ok(files::is_dir(&path)))?;

    let is_file = ctx.create_function(|_, path: String| Ok(files::is_file(&path)))?;

    let cd = ctx.create_function(|ctx, path: String| utils::treat_error(ctx, files::cd(&path)))?;

    let pwd = ctx.create_function(|ctx, ()| utils::treat_error(ctx, files::pwd()))?;

    let rn = ctx.create_function(|ctx, (origin, destiny): (String, String)| {
        utils::treat_error(ctx, files::rn(&origin, &destiny))
    })?;

    let cp = ctx.create_function(|ctx, (origin, destiny): (String, String)| {
        utils::treat_error(ctx, files::cp(&origin, &destiny))
    })?;

    let cp_tmp = ctx.create_function(|ctx, (origin, destiny): (String, String)| {
        utils::treat_error(ctx, files::cp_tmp(&origin, &destiny))
    })?;

    let mv = ctx.create_function(|ctx, (origin, destiny): (String, String)| {
        utils::treat_error(ctx, files::mv(&origin, &destiny))
    })?;

    let rm = ctx.create_function(|ctx, path: String| utils::treat_error(ctx, files::rm(&path)))?;

    let read =
        ctx.create_function(|ctx, path: String| utils::treat_error(ctx, files::read(&path)))?;

    let mk_dir =
        ctx.create_function(|ctx, path: String| utils::treat_error(ctx, files::mk_dir(&path)))?;

    let touch =
        ctx.create_function(|ctx, path: String| utils::treat_error(ctx, files::touch(&path)))?;

    let write = ctx.create_function(|ctx, (path, contents): (String, String)| {
        utils::treat_error(ctx, files::write(&path, &contents))
    })?;

    let append = ctx.create_function(|ctx, (path, contents): (String, String)| {
        utils::treat_error(ctx, files::append(&path, &contents))
    })?;

    let path_ext =
        ctx.create_function(|ctx, path: String| utils::treat_error(ctx, files::path_ext(&path)))?;

    let path_name =
        ctx.create_function(|ctx, path: String| utils::treat_error(ctx, files::path_name(&path)))?;

    let path_stem =
        ctx.create_function(|ctx, path: String| utils::treat_error(ctx, files::path_stem(&path)))?;

    let path_absolute = ctx.create_function(|ctx, path: String| {
        utils::treat_error(ctx, files::path_absolute(&path))
    })?;

    let path_relative = ctx.create_function(|ctx, (path, base): (String, String)| {
        utils::treat_error(ctx, files::path_relative(&path, &base))
    })?;

    let path_parent = ctx
        .create_function(|ctx, path: String| utils::treat_error(ctx, files::path_parent(&path)))?;

    let path_parent_find = ctx.create_function(|ctx, (path, with_name): (String, String)| {
        utils::treat_error(ctx, files::path_parent_find(&path, &with_name))
    })?;

    let path_join = ctx.create_function(|ctx, (path, child): (String, String)| {
        utils::treat_error(ctx, files::path_join(&path, &child))
    })?;

    let path_list =
        ctx.create_function(|ctx, path: String| utils::treat_error(ctx, files::path_list(&path)))?;

    let path_list_subs = ctx.create_function(|ctx, path: String| {
        utils::treat_error(ctx, files::path_list_subs(&path))
    })?;

    let path_list_dirs = ctx.create_function(|ctx, path: String| {
        utils::treat_error(ctx, files::path_list_dirs(&path))
    })?;

    let path_list_dirs_subs = ctx.create_function(|ctx, path: String| {
        utils::treat_error(ctx, files::path_list_dirs_subs(&path))
    })?;

    let path_list_files = ctx.create_function(|ctx, path: String| {
        utils::treat_error(ctx, files::path_list_files(&path))
    })?;

    let path_list_files_subs = ctx.create_function(|ctx, path: String| {
        utils::treat_error(ctx, files::path_list_files_subs(&path))
    })?;

    let path_list_files_ext = ctx.create_function(|ctx, (path, ext): (String, String)| {
        utils::treat_error(ctx, files::path_list_files_ext(&path, &ext))
    })?;

    let path_list_files_exts =
        ctx.create_function(|ctx, (path, exts): (String, Vec<String>)| {
            let exts: Vec<_> = exts.iter().map(String::as_ref).collect();
            utils::treat_error(ctx, files::path_list_files_exts(&path, &exts))
        })?;

    let path_list_files_ext_subs = ctx.create_function(|ctx, (path, ext): (String, String)| {
        utils::treat_error(ctx, files::path_list_files_ext_subs(&path, &ext))
    })?;

    let path_list_files_exts_subs =
        ctx.create_function(|ctx, (path, exts): (String, Vec<String>)| {
            let exts: Vec<_> = exts.iter().map(String::as_str).collect();
            utils::treat_error(ctx, files::path_list_files_exts_subs(&path, &exts))
        })?;

    liz.set("has", has)?;
    liz.set("is_dir", is_dir)?;
    liz.set("is_file", is_file)?;
    liz.set("cd", cd)?;
    liz.set("pwd", pwd)?;
    liz.set("rn", rn)?;
    liz.set("cp", cp)?;
    liz.set("cp_tmp", cp_tmp)?;
    liz.set("mv", mv)?;
    liz.set("rm", rm)?;
    liz.set("read", read)?;
    
    // TODO - Removes this undescore because it should be equals on linux systems.
    liz.set("mk_dir", mk_dir)?;
    
    liz.set("touch", touch)?;
    liz.set("write", write)?;
    liz.set("append", append)?;
    liz.set("exe_ext", files::exe_ext())?;
    liz.set("path_sep", files::path_sep())?;
    liz.set("path_ext", path_ext)?;
    liz.set("path_name", path_name)?;
    liz.set("path_stem", path_stem)?;
    liz.set("path_absolute", path_absolute)?;
    liz.set("path_relative", path_relative)?;
    liz.set("path_parent", path_parent)?;
    liz.set("path_parent_find", path_parent_find)?;
    liz.set("path_join", path_join)?;
    liz.set("path_list", path_list)?;
    liz.set("path_list_subs", path_list_subs)?;
    liz.set("path_list_dirs", path_list_dirs)?;
    liz.set("path_list_dirs_subs", path_list_dirs_subs)?;
    liz.set("path_list_files", path_list_files)?;
    liz.set("path_list_files_subs", path_list_files_subs)?;
    liz.set("path_list_files_ext", path_list_files_ext)?;
    liz.set("path_list_files_exts", path_list_files_exts)?;
    liz.set("path_list_files_ext_subs", path_list_files_ext_subs)?;
    liz.set("path_list_files_exts_subs", path_list_files_exts_subs)?;

    Ok(())
}

fn inject_texts<'a>(ctx: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let ask =
        ctx.create_function(|ctx, message: String| utils::treat_error(ctx, texts::ask(&message)))?;

    let ask_int = ctx.create_function(|ctx, message: String| {
        utils::treat_error(ctx, texts::ask_int(&message))
    })?;

    let ask_float = ctx.create_function(|ctx, message: String| {
        utils::treat_error(ctx, texts::ask_float(&message))
    })?;

    let ask_bool = ctx.create_function(|ctx, message: String| {
        utils::treat_error(ctx, texts::ask_bool(&message))
    })?;

    let trim = ctx.create_function(|_, text: String| Ok(texts::trim(&text)))?;

    let tolower = ctx.create_function(|_, text: String| Ok(texts::tolower(&text)))?;

    let toupper = ctx.create_function(|_, text: String| Ok(texts::toupper(&text)))?;
    
    let contains = ctx.create_function(|_, (text, part): (String, String)| {
        Ok(texts::contains(&text, &part))
    })?;

    let find = ctx.create_function(|_, (text, part): (String, String)| {
        Ok(texts::find(&text, &part))
    })?;

    let starts_with = ctx.create_function(|_, (text, contents): (String, String)| {
        Ok(texts::starts_with(&text, &contents))
    })?;

    let ends_with = ctx.create_function(|_, (text, contents): (String, String)| {
        Ok(texts::ends_with(&text, &contents))
    })?;

    let text_path_find = ctx.create_function(|ctx, (path, contents): (String, String)| {
        utils::treat_error(ctx, texts::text_path_find(&path, &contents))
    })?;

    let text_dir_find = ctx.create_function(|ctx, (path, contents): (String, String)| {
        utils::treat_error(ctx, texts::text_dir_find(&path, &contents))
    })?;

    let text_file_find = ctx.create_function(|ctx, (path, contents): (String, String)| {
        utils::treat_error(ctx, texts::text_file_find(&path, &contents))
    })?;

    let text_files_find =
        ctx.create_function(|ctx, (paths, contents): (Vec<String>, String)| {
            utils::treat_error(ctx, texts::text_files_find(paths, contents))
        })?;

    liz.set("ask", ask)?;
    liz.set("ask_int", ask_int)?;
    liz.set("ask_float", ask_float)?;
    liz.set("ask_bool", ask_bool)?;
    liz.set("trim", trim)?;
    liz.set("tolower", tolower)?;
    liz.set("toupper", toupper)?;
    liz.set("contains", contains)?;
    liz.set("find", find)?;
    liz.set("starts_with", starts_with)?;
    liz.set("ends_with", ends_with)?;
    liz.set("text_path_find", text_path_find)?;
    liz.set("text_dir_find", text_dir_find)?;
    liz.set("text_file_find", text_file_find)?;
    liz.set("text_files_find", text_files_find)?;

    Ok(())
}

fn inject_trans<'a>(ctx: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let get = ctx.create_function(
        |ctx, (url, headers): (String, Option<HashMap<String, String>>)| {
            utils::treat_error(ctx, trans::get(&url, headers))
        },
    )?;

    let post = ctx.create_function(
        |ctx, (url, text, headers): (String, String, Option<HashMap<String, String>>)| {
            utils::treat_error(ctx, trans::post(&url, text, headers))
        },
    )?;

    liz.set("get", get)?;
    liz.set("post", post)?;

    Ok(())
}
