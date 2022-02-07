use rlua::{Context, Table};

use crate::liz_files;
use crate::utils;

use crate::LizError;

pub fn inject_files<'a>(ctx: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let has = ctx.create_function(|_, path: String| Ok(liz_files::has(&path)))?;

    let is_dir = ctx.create_function(|_, path: String| Ok(liz_files::is_dir(&path)))?;

    let is_file = ctx.create_function(|_, path: String| Ok(liz_files::is_file(&path)))?;

    let is_absolute = ctx.create_function(|_, path: String| Ok(liz_files::is_absolute(&path)))?;

    let is_relative = ctx.create_function(|_, path: String| Ok(liz_files::is_relative(&path)))?;

    let is_symlink = ctx.create_function(|_, path: String| Ok(liz_files::is_symlink(&path)))?;

    let cd =
        ctx.create_function(|ctx, path: String| utils::treat_error(ctx, liz_files::cd(&path)))?;

    let pwd = ctx.create_function(|ctx, ()| utils::treat_error(ctx, liz_files::pwd()))?;

    let rn = ctx.create_function(|ctx, (origin, destiny): (String, String)| {
        utils::treat_error(ctx, liz_files::rn(&origin, &destiny))
    })?;

    let cp = ctx.create_function(|ctx, (origin, destiny): (String, String)| {
        utils::treat_error(ctx, liz_files::cp(&origin, &destiny))
    })?;

    let cp_tmp = ctx.create_function(|ctx, (origin, destiny): (String, String)| {
        utils::treat_error(ctx, liz_files::cp_tmp(&origin, &destiny))
    })?;

    let mv = ctx.create_function(|ctx, (origin, destiny): (String, String)| {
        utils::treat_error(ctx, liz_files::mv(&origin, &destiny))
    })?;

    let rm =
        ctx.create_function(|ctx, path: String| utils::treat_error(ctx, liz_files::rm(&path)))?;

    let mkdir =
        ctx.create_function(|ctx, path: String| utils::treat_error(ctx, liz_files::mkdir(&path)))?;

    let touch =
        ctx.create_function(|ctx, path: String| utils::treat_error(ctx, liz_files::touch(&path)))?;

    let exe_ext = ctx.create_function(|_, ()| Ok(liz_files::exe_ext()))?;

    let os_sep = ctx.create_function(|_, ()| Ok(String::from(*liz_files::os_sep())))?;

    let path_sep = ctx.create_function(|_, path: String| Ok(liz_files::path_sep(&path)))?;

    let path_parts = ctx.create_function(|_, path: String| {
        Ok(liz_files::path_parts(&path)
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>())
    })?;

    let path_parts_join = ctx.create_function(|_, parts: Vec<String>| {
        Ok(liz_files::path_parts_join(
            parts
                .iter()
                .map(String::as_str)
                .collect::<Vec<&str>>()
                .as_slice(),
        ))
    })?;

    let path_name =
        ctx.create_function(|_, path: String| Ok(String::from(liz_files::path_name(&path))))?;

    let path_stem =
        ctx.create_function(|_, path: String| Ok(String::from(liz_files::path_stem(&path))))?;

    let path_ext =
        ctx.create_function(|_, path: String| Ok(String::from(liz_files::path_ext(&path))))?;

    let path_absolute = ctx.create_function(|ctx, path: String| {
        utils::treat_error(ctx, liz_files::path_absolute(&path))
    })?;

    let path_relative = ctx.create_function(|ctx, (path, base): (String, String)| {
        utils::treat_error(ctx, liz_files::path_relative(&path, &base))
    })?;

    let path_walk = ctx.create_function(|ctx, path: String| {
        utils::treat_error(ctx, liz_files::path_walk(&path))
    })?;

    let path_parent = ctx.create_function(|ctx, path: String| {
        utils::treat_error(ctx, liz_files::path_parent(&path))
    })?;

    let path_parent_find = ctx.create_function(|ctx, (path, with_name): (String, String)| {
        utils::treat_error(ctx, liz_files::path_parent_find(&path, &with_name))
    })?;

    let path_join = ctx.create_function(|ctx, (path, child): (String, String)| {
        utils::treat_error(ctx, liz_files::path_join(&path, &child))
    })?;

    let path_list = ctx.create_function(|ctx, path: String| {
        utils::treat_error(ctx, liz_files::path_list(&path))
    })?;

    let path_list_in = ctx.create_function(|ctx, path: String| {
        utils::treat_error(ctx, liz_files::path_list_in(&path))
    })?;

    let path_list_dirs = ctx.create_function(|ctx, path: String| {
        utils::treat_error(ctx, liz_files::path_list_dirs(&path))
    })?;

    let path_list_dirs_in = ctx.create_function(|ctx, path: String| {
        utils::treat_error(ctx, liz_files::path_list_dirs_in(&path))
    })?;

    let path_list_files = ctx.create_function(|ctx, path: String| {
        utils::treat_error(ctx, liz_files::path_list_files(&path))
    })?;

    let path_list_files_in = ctx.create_function(|ctx, path: String| {
        utils::treat_error(ctx, liz_files::path_list_files_in(&path))
    })?;

    let path_list_files_ext = ctx.create_function(|ctx, (path, ext): (String, String)| {
        utils::treat_error(ctx, liz_files::path_list_files_ext(&path, &ext))
    })?;

    let path_list_files_ext_in = ctx.create_function(|ctx, (path, ext): (String, String)| {
        utils::treat_error(ctx, liz_files::path_list_files_ext_in(&path, &ext))
    })?;

    let path_list_files_exts =
        ctx.create_function(|ctx, (path, exts): (String, Vec<String>)| {
            utils::treat_error(
                ctx,
                liz_files::path_list_files_exts(
                    &path,
                    exts.iter()
                        .map(String::as_str)
                        .collect::<Vec<&str>>()
                        .as_slice(),
                ),
            )
        })?;

    let path_list_files_exts_in =
        ctx.create_function(|ctx, (path, exts): (String, Vec<String>)| {
            utils::treat_error(
                ctx,
                liz_files::path_list_files_exts_in(
                    &path,
                    exts.iter()
                        .map(String::as_str)
                        .collect::<Vec<&str>>()
                        .as_slice(),
                ),
            )
        })?;

    liz.set("has", has)?;
    liz.set("is_dir", is_dir)?;
    liz.set("is_file", is_file)?;
    liz.set("is_absolute", is_absolute)?;
    liz.set("is_relative", is_relative)?;
    liz.set("is_symlink", is_symlink)?;
    liz.set("cd", cd)?;
    liz.set("pwd", pwd)?;
    liz.set("rn", rn)?;
    liz.set("cp", cp)?;
    liz.set("cp_tmp", cp_tmp)?;
    liz.set("mv", mv)?;
    liz.set("rm", rm)?;
    liz.set("mkdir", mkdir)?;
    liz.set("touch", touch)?;
    liz.set("exe_ext", exe_ext)?;
    liz.set("os_sep", os_sep)?;
    liz.set("path_sep", path_sep)?;
    liz.set("path_parts", path_parts)?;
    liz.set("path_parts_join", path_parts_join)?;
    liz.set("path_name", path_name)?;
    liz.set("path_stem", path_stem)?;
    liz.set("path_ext", path_ext)?;
    liz.set("path_absolute", path_absolute)?;
    liz.set("path_relative", path_relative)?;
    liz.set("path_walk", path_walk)?;
    liz.set("path_parent", path_parent)?;
    liz.set("path_parent_find", path_parent_find)?;
    liz.set("path_join", path_join)?;
    liz.set("path_list", path_list)?;
    liz.set("path_list_in", path_list_in)?;
    liz.set("path_list_dirs", path_list_dirs)?;
    liz.set("path_list_dirs_in", path_list_dirs_in)?;
    liz.set("path_list_files", path_list_files)?;
    liz.set("path_list_files_in", path_list_files_in)?;
    liz.set("path_list_files_ext", path_list_files_ext)?;
    liz.set("path_list_files_ext_in", path_list_files_ext_in)?;
    liz.set("path_list_files_exts", path_list_files_exts)?;
    liz.set("path_list_files_exts_in", path_list_files_exts_in)?;

    Ok(())
}
