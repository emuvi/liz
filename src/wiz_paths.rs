use rlua::{Context, Table};

use crate::liz_paths;
use crate::utils;

use crate::LizError;

pub fn inject_paths<'a>(lane: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let has = lane.create_function(|_, path: String| Ok(liz_paths::has(&path)))?;

    let is_dir = lane.create_function(|_, path: String| Ok(liz_paths::is_dir(&path)))?;

    let is_file = lane.create_function(|_, path: String| Ok(liz_paths::is_file(&path)))?;

    let is_absolute = lane.create_function(|_, path: String| Ok(liz_paths::is_absolute(&path)))?;

    let is_relative = lane.create_function(|_, path: String| Ok(liz_paths::is_relative(&path)))?;

    let is_symlink = lane.create_function(|_, path: String| Ok(liz_paths::is_symlink(&path)))?;

    let cd = lane.create_function(|_, path: String| utils::treat_error(liz_paths::cd(&path)))?;

    let pwd = lane.create_function(|_, ()| utils::treat_error(liz_paths::pwd()))?;

    let rn = lane.create_function(|_, (origin, destiny): (String, String)| {
        utils::treat_error(liz_paths::rn(&origin, &destiny))
    })?;

    let cp = lane.create_function(|_, (origin, destiny): (String, String)| {
        utils::treat_error(liz_paths::cp(&origin, &destiny))
    })?;

    let cp_tmp = lane.create_function(|_, (origin, destiny): (String, String)| {
        utils::treat_error(liz_paths::cp_tmp(&origin, &destiny))
    })?;

    let mv = lane.create_function(|_, (origin, destiny): (String, String)| {
        utils::treat_error(liz_paths::mv(&origin, &destiny))
    })?;

    let rm = lane.create_function(|_, path: String| utils::treat_error(liz_paths::rm(&path)))?;

    let mkdir =
        lane.create_function(|_, path: String| utils::treat_error(liz_paths::mkdir(&path)))?;

    let touch =
        lane.create_function(|_, path: String| utils::treat_error(liz_paths::touch(&path)))?;

    let os_sep = lane.create_function(|_, ()| Ok(String::from(*liz_paths::os_sep())))?;

    let path_sep = lane.create_function(|_, path: String| Ok(liz_paths::path_sep(&path)))?;

    let path_parts = lane.create_function(|_, path: String| {
        Ok(liz_paths::path_parts(&path)
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>())
    })?;

    let path_parts_join = lane.create_function(|_, parts: Vec<String>| {
        Ok(liz_paths::path_parts_join(
            parts
                .iter()
                .map(String::as_str)
                .collect::<Vec<&str>>()
                .as_slice(),
        ))
    })?;

    let path_name =
        lane.create_function(|_, path: String| Ok(String::from(liz_paths::path_name(&path))))?;

    let path_stem =
        lane.create_function(|_, path: String| Ok(String::from(liz_paths::path_stem(&path))))?;

    let path_ext =
        lane.create_function(|_, path: String| Ok(String::from(liz_paths::path_ext(&path))))?;

    let path_ext_is = lane.create_function(|_, (path, ext): (String, String)| {
        Ok(liz_paths::path_ext_is(&path, &ext))
    })?;

    let path_ext_is_on = lane.create_function(|_, (path, exts): (String, Vec<String>)| {
        Ok(liz_paths::path_ext_is_on(&path, exts.as_slice()))
    })?;

    let path_absolute = lane
        .create_function(|_, path: String| utils::treat_error(liz_paths::path_absolute(&path)))?;

    let path_relative = lane.create_function(|_, (path, base): (String, String)| {
        utils::treat_error(liz_paths::path_relative(&path, &base))
    })?;

    let path_walk =
        lane.create_function(|_, path: String| utils::treat_error(liz_paths::path_walk(&path)))?;

    let path_parent =
        lane.create_function(|_, path: String| utils::treat_error(liz_paths::path_parent(&path)))?;

    let path_parent_find = lane.create_function(|_, (path, with_name): (String, String)| {
        utils::treat_error(liz_paths::path_parent_find(&path, &with_name))
    })?;

    let path_join = lane.create_function(|_, (path, child): (String, String)| {
        utils::treat_error(liz_paths::path_join(&path, &child))
    })?;

    let path_join_if_relative = lane.create_function(|_, (base, path): (String, String)| {
        utils::treat_error(liz_paths::path_join_if_relative(&base, &path))
    })?;

    let path_list =
        lane.create_function(|_, path: String| utils::treat_error(liz_paths::path_list(&path)))?;

    let path_list_in =
        lane.create_function(|_, path: String| utils::treat_error(liz_paths::path_list_in(&path)))?;

    let path_list_dirs = lane
        .create_function(|_, path: String| utils::treat_error(liz_paths::path_list_dirs(&path)))?;

    let path_list_dirs_in = lane.create_function(|_, path: String| {
        utils::treat_error(liz_paths::path_list_dirs_in(&path))
    })?;

    let path_list_files = lane
        .create_function(|_, path: String| utils::treat_error(liz_paths::path_list_files(&path)))?;

    let path_list_files_in = lane.create_function(|_, path: String| {
        utils::treat_error(liz_paths::path_list_files_in(&path))
    })?;

    let path_list_files_ext = lane.create_function(|_, (path, ext): (String, String)| {
        utils::treat_error(liz_paths::path_list_files_ext(&path, &ext))
    })?;

    let path_list_files_ext_in = lane.create_function(|_, (path, ext): (String, String)| {
        utils::treat_error(liz_paths::path_list_files_ext_in(&path, &ext))
    })?;

    let path_list_files_exts = lane.create_function(|_, (path, exts): (String, Vec<String>)| {
        utils::treat_error(liz_paths::path_list_files_exts(
            &path,
            exts.iter()
                .map(String::as_str)
                .collect::<Vec<&str>>()
                .as_slice(),
        ))
    })?;

    let path_list_files_exts_in =
        lane.create_function(|_, (path, exts): (String, Vec<String>)| {
            utils::treat_error(liz_paths::path_list_files_exts_in(
                &path,
                exts.iter()
                    .map(String::as_str)
                    .collect::<Vec<&str>>()
                    .as_slice(),
            ))
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
    liz.set("os_sep", os_sep)?;
    liz.set("path_sep", path_sep)?;
    liz.set("path_parts", path_parts)?;
    liz.set("path_parts_join", path_parts_join)?;
    liz.set("path_name", path_name)?;
    liz.set("path_stem", path_stem)?;
    liz.set("path_ext", path_ext)?;
    liz.set("path_ext_is", path_ext_is)?;
    liz.set("path_ext_is_on", path_ext_is_on)?;
    liz.set("path_absolute", path_absolute)?;
    liz.set("path_relative", path_relative)?;
    liz.set("path_walk", path_walk)?;
    liz.set("path_parent", path_parent)?;
    liz.set("path_parent_find", path_parent_find)?;
    liz.set("path_join", path_join)?;
    liz.set("path_join_if_relative", path_join_if_relative)?;
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
