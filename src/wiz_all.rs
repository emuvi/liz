use rlua::{Context, MultiValue, Value};
use rubx::rux_paths;
use rubx::{rux_dbg_bleb, rux_dbg_erro, rux_dbg_step};

use crate::wiz_codes;
use crate::wiz_fires;
use crate::wiz_forms;
use crate::wiz_parse;
use crate::wiz_group;
use crate::wiz_logic;
use crate::wiz_paths;
use crate::wiz_texts;
use crate::wiz_times;
use crate::wiz_winds;

use crate::liz_codes;
use crate::utils;
use crate::LizError;

pub fn inject_all(
    lane: Context,
    path: &str,
    args: &Option<Vec<String>>,
) -> Result<String, LizError> {
    rux_dbg_step!(path, args);
    let liz = lane.create_table().map_err(|err| rux_dbg_erro!(err))?;
    liz.set("args", args.clone()).map_err(|err| rux_dbg_erro!(err))?;

    let suit_path = liz_codes::liz_suit_path(path).map_err(|err| rux_dbg_bleb!(err))?;
    rux_dbg_step!(suit_path);

    let suit_path = if rux_paths::is_symlink(&suit_path) {
        rux_paths::path_walk(&suit_path).map_err(|err| rux_dbg_bleb!(err))?
    } else {
        suit_path
    };
    rux_dbg_step!(suit_path);

    let rise_wd = rux_paths::wd().map_err(|err| rux_dbg_bleb!(err))?;
    rux_dbg_step!(rise_wd);

    let rise_dir = rux_paths::path_parent(&suit_path).map_err(|err| rux_dbg_bleb!(err))?;
    rux_dbg_step!(rise_dir);
    utils::put_stack_dir(&lane, &liz, rise_dir.clone()).map_err(|err| rux_dbg_bleb!(err))?;

    let rise_path = rux_paths::path_absolute(&suit_path).map_err(|err| rux_dbg_bleb!(err))?;
    rux_dbg_step!(rise_path);

    liz.set("rise_wd", rise_wd).map_err(|err| rux_dbg_erro!(err))?;
    liz.set("rise_dir", rise_dir).map_err(|err| rux_dbg_erro!(err))?;
    liz.set("rise_path", rise_path.clone())
        .map_err(|err| rux_dbg_erro!(err))?;

    let print_stack_dir =
        lane.create_function(|lane, ()| utils::treat_error(utils::print_stack_dir(lane)))?;

    let get_stacked_dir =
        lane.create_function(|lane, ()| utils::treat_error(utils::get_stacked_dir(lane)))?;

    let to_json_multi = lane.create_function(|_, values: MultiValue| {
        utils::treat_error(utils::to_json_multi(values))
    })?;

    let to_json =
        lane.create_function(|_, value: Value| utils::treat_error(utils::to_json(value)))?;

    let from_json = lane.create_function(|lane, source: String| {
        utils::treat_error(utils::from_json(lane, source))
    })?;

    liz.set("print_stack_dir", print_stack_dir)?;
    liz.set("get_stacked_dir", get_stacked_dir)?;
    liz.set("to_json_multi", to_json_multi)?;
    liz.set("to_json", to_json)?;
    liz.set("from_json", from_json)?;

    wiz_codes::inject_codes(lane, &liz)?;
    wiz_fires::inject_execs(lane, &liz)?;
    wiz_forms::inject_forms(lane, &liz)?;
    wiz_group::inject_group(lane, &liz)?;
    wiz_logic::inject_logic(lane, &liz)?;
    wiz_parse::inject_parse(lane, &liz)?;
    wiz_paths::inject_paths(lane, &liz)?;
    wiz_texts::inject_texts(lane, &liz)?;
    wiz_times::inject_times(lane, &liz)?;
    wiz_winds::inject_winds(lane, &liz)?;

    let globals = lane.globals();
    globals.set("Liz", liz)?;

    Ok(rise_path)
}
