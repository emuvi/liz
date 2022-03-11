use rlua::{Context, Table};

use crate::liz_codes;
use crate::utils;

use crate::LizError;

pub fn inject_codes<'a>(lane: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let edit = lane.create_function(|_, ()| Ok(liz_codes::edit()))?;

    let code = lane.create_function(|_, source: String| Ok(liz_codes::code(source)))?;

    let desk = lane.create_function(|_, terms: Vec<String>| Ok(liz_codes::desk(terms)))?;

    let liz_suit_path = lane
        .create_function(|_, path: String| utils::treat_error(liz_codes::liz_suit_path(&path)))?;

    let is_lizs_update = lane.create_function(|_, ()| Ok(liz_codes::is_lizs_update()))?;
    
    let set_lizs_update = lane.create_function(|_, to: bool| Ok(liz_codes::set_lizs_update(to)))?;

    let gotta_lizs = lane.create_function(|_, path: String| utils::treat_error(liz_codes::gotta_lizs(&path)))?;

    let get_lizs = lane.create_function(|_, path: String| utils::treat_error(liz_codes::get_lizs(&path)))?;

    let get_lizs_path_pos = lane.create_function(|_, path: String| Ok(liz_codes::get_lizs_path_pos(&path)))?;

    let get_lizs_file = lane.create_function(|_, (net_path, local_path): (String, String)| utils::treat_error(liz_codes::get_lizs_file(&net_path, &local_path)))?;
    
    let git_root_find = lane
        .create_function(|_, path: String| utils::treat_error(liz_codes::git_root_find(&path)))?;

    let git_is_ignored = lane
        .create_function(|_, path: String| utils::treat_error(liz_codes::git_is_ignored(&path)))?;

    let git_has_changes = lane
        .create_function(|_, path: String| utils::treat_error(liz_codes::git_has_changes(&path)))?;

    liz.set("code", code)?;
    liz.set("edit", edit)?;
    liz.set("desk", desk)?;
    liz.set("liz_suit_path", liz_suit_path)?;
    liz.set("is_lizs_update", is_lizs_update)?;
    liz.set("set_lizs_update", set_lizs_update)?;
    liz.set("gotta_lizs", gotta_lizs)?;
    liz.set("get_lizs", get_lizs)?;
    liz.set("get_lizs_path_pos", get_lizs_path_pos)?;
    liz.set("get_lizs_file", get_lizs_file)?;
    liz.set("git_root_find", git_root_find)?;
    liz.set("git_is_ignored", git_is_ignored)?;
    liz.set("git_has_changes", git_has_changes)?;

    Ok(())
}
