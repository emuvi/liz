use rlua::{Context, MultiValue, Value};

use crate::liz_paths;
use crate::wiz_codes;
use crate::wiz_fires;
use crate::wiz_paths;
use crate::wiz_texts;
use crate::wiz_times;
use crate::wiz_winds;

use crate::liz_debug::{dbg_err, dbg_stp};
use crate::utils;
use crate::LizError;

pub fn inject_all(lane: Context, path: &str, args: &Option<Vec<String>>) -> Result<(), LizError> {
    dbg_stp!(path, args);
    let liz = lane.create_table()?;
    liz.set("args", args.clone())?;

    let path = utils::add_liz_extension(path);
    dbg_stp!(path);
    let path = if liz_paths::is_symlink(&path) {
        liz_paths::path_walk(&path).map_err(|err| dbg_err!(err, path))?
    } else {
        path
    };
    dbg_stp!(path);

    let rise_pwd = liz_paths::pwd().map_err(|err| dbg_err!(err))?;
    dbg_stp!(rise_pwd);
    liz.set("rise_pwd", rise_pwd)?;

    let rise_dir = liz_paths::path_parent(&path).map_err(|err| dbg_err!(err, path))?;
    dbg_stp!(rise_dir);
    utils::put_stack_dir(&lane, &liz, rise_dir.clone()).map_err(|err| dbg_err!(err, rise_dir))?;
    liz.set("rise_dir", rise_dir)?;

    let rise_path = liz_paths::path_absolute(&path).map_err(|err| dbg_err!(err, path))?;
    dbg_stp!(rise_path);
    liz.set("rise_path", rise_path)?;

    let print_stack_dir =
        lane.create_function(|lane, ()| utils::treat_error(utils::print_stack_dir(lane)))?;

    let last_stack_dir =
        lane.create_function(|lane, ()| utils::treat_error(utils::last_stack_dir(lane)))?;

    let to_json_multi = lane.create_function(|_, values: MultiValue| {
        utils::treat_error(utils::to_json_multi(values))
    })?;

    let to_json =
        lane.create_function(|_, value: Value| utils::treat_error(utils::to_json(value)))?;

    let from_json = lane.create_function(|lane, source: String| {
        utils::treat_error(utils::from_json(lane, source))
    })?;

    liz.set("print_stack_dir", print_stack_dir)?;
    liz.set("last_stack_dir", last_stack_dir)?;
    liz.set("to_json_multi", to_json_multi)?;
    liz.set("to_json", to_json)?;
    liz.set("from_json", from_json)?;

    wiz_codes::inject_codes(lane, &liz)?;
    wiz_fires::inject_execs(lane, &liz)?;
    wiz_paths::inject_paths(lane, &liz)?;
    wiz_texts::inject_texts(lane, &liz)?;
    wiz_times::inject_times(lane, &liz)?;
    wiz_winds::inject_winds(lane, &liz)?;

    let globals = lane.globals();
    globals.set("liz", liz)?;

    Ok(())
}
