use rlua::{Context, Table};

use crate::liz_codes;
use crate::utils;

use crate::LizError;

pub fn inject_codes<'a>(lane: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let code =
        lane.create_function(|lane, path: String| utils::treat_error(lane, liz_codes::code(&path)))?;

    let git_root_find = lane.create_function(|lane, path: String| {
        utils::treat_error(lane, liz_codes::git_root_find(&path))
    })?;

    let git_is_ignored = lane.create_function(|lane, path: String| {
        utils::treat_error(lane, liz_codes::git_is_ignored(&path))
    })?;

    let git_has_changes = lane.create_function(|lane, path: String| {
        utils::treat_error(lane, liz_codes::git_has_changes(&path))
    })?;

    liz.set("code", code)?;
    liz.set("git_root_find", git_root_find)?;
    liz.set("git_is_ignored", git_is_ignored)?;
    liz.set("git_has_changes", git_has_changes)?;

    Ok(())
}
