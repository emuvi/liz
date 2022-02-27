use rlua::{Context, Table};

use crate::liz_codes;
use crate::utils;

use crate::LizError;

pub fn inject_codes<'a>(lane: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let code = lane.create_function(|_, source: String| Ok(liz_codes::code(&source)))?;

    let edit = lane.create_function(|_, ()| Ok(liz_codes::edit()))?;

    let desk = lane.create_function(|_, terms: Vec<String>| Ok(liz_codes::desk(terms)))?;

    let form = lane.create_function(|_, part: String| Ok(liz_codes::form(&part)))?;

    let liz_suit_path = lane
        .create_function(|_, path: String| utils::treat_error(liz_codes::liz_suit_path(&path)))?;

    let is_update_lizs = lane.create_function(|_, ()| Ok(liz_codes::is_update_lizs()))?;
    
    let set_update_lizs = lane.create_function(|_, to: bool| Ok(liz_codes::set_update_lizs(to)))?;

    let gotta_lizs = lane.create_function(|_, path: String| utils::treat_error(liz_codes::gotta_lizs(&path)))?;
    
    let git_root_find = lane
        .create_function(|_, path: String| utils::treat_error(liz_codes::git_root_find(&path)))?;

    let git_is_ignored = lane
        .create_function(|_, path: String| utils::treat_error(liz_codes::git_is_ignored(&path)))?;

    let git_has_changes = lane
        .create_function(|_, path: String| utils::treat_error(liz_codes::git_has_changes(&path)))?;

    liz.set("code", code)?;
    liz.set("edit", edit)?;
    liz.set("desk", desk)?;
    liz.set("form", form)?;
    liz.set("liz_suit_path", liz_suit_path)?;
    liz.set("is_update_lizs", is_update_lizs)?;
    liz.set("set_update_lizs", set_update_lizs)?;
    liz.set("gotta_lizs", gotta_lizs)?;
    liz.set("git_root_find", git_root_find)?;
    liz.set("git_is_ignored", git_is_ignored)?;
    liz.set("git_has_changes", git_has_changes)?;

    Ok(())
}
