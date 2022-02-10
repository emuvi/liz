use rlua::{Context, MultiValue, Value};

use crate::liz_paths;
use crate::wiz_codes;
use crate::wiz_execs;
use crate::wiz_paths;
use crate::wiz_texts;
use crate::wiz_trans;

use crate::utils::{self, dbg_err};
use crate::LizError;

pub fn inject_all(lane: Context, path: &str, args: Option<Vec<String>>) -> Result<(), LizError> {
    let liz = lane.create_table()?;
    liz.set("args", args)?;

    let path = utils::add_liz_extension(path);
    let path = if liz_paths::is_symlink(&path) {
        liz_paths::path_walk(&path).map_err(|err| dbg_err!(err, "path_walk", path))?
    } else {
        path
    };

    let rise_pwd = liz_paths::pwd().map_err(|err| dbg_err!(err, "pwd"))?;
    liz.set("rise_pwd", rise_pwd)?;

    let rise_dir = liz_paths::path_parent(&path).map_err(|err| dbg_err!(err, "path_parent", path))?;
    utils::put_stack_dir(&lane, &liz, rise_dir.clone())
        .map_err(|err| dbg_err!(err, "put_stack_dir", rise_dir))?;
    liz.set("rise_dir", rise_dir)?;

    let rise_path =
        liz_paths::path_absolute(&path).map_err(|err| dbg_err!(err, "path_absolute", path))?;
    liz.set("rise_path", rise_path)?;

    let print_stack_dir =
        lane.create_function(|lane, ()| utils::treat_error(lane, utils::print_stack_dir(lane)))?;

    let last_stack_dir =
        lane.create_function(|lane, ()| utils::treat_error(lane, utils::last_stack_dir(lane)))?;

    let to_json_multi = lane.create_function(|lane, values: MultiValue| {
        utils::treat_error(lane, utils::to_json_multi(values))
    })?;

    let to_json =
        lane.create_function(|lane, value: Value| utils::treat_error(lane, utils::to_json(value)))?;

    let from_json = lane.create_function(|lane, source: String| {
        utils::treat_error(lane, utils::from_json(lane, source))
    })?;

    liz.set("print_stack_dir", print_stack_dir)?;
    liz.set("last_stack_dir", last_stack_dir)?;
    liz.set("to_json_multi", to_json_multi)?;
    liz.set("to_json", to_json)?;
    liz.set("from_json", from_json)?;

    wiz_codes::inject_codes(lane, &liz)?;
    wiz_execs::inject_execs(lane, &liz)?;
    wiz_paths::inject_paths(lane, &liz)?;
    wiz_texts::inject_texts(lane, &liz)?;
    wiz_trans::inject_trans(lane, &liz)?;

    let globals = lane.globals();
    globals.set("liz", liz)?;

    Ok(())
}
