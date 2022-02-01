use rlua::{Context, Table};

use crate::liz_codes;
use crate::utils;

use crate::LizError;

pub fn inject_codes<'a>(ctx: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let git_root_find = ctx.create_function(|ctx, path: String| {
        utils::treat_error(ctx, liz_codes::git_root_find(&path))
    })?;

    let git_is_ignored = ctx.create_function(|ctx, path: String| {
        utils::treat_error(ctx, liz_codes::git_is_ignored(&path))
    })?;

    liz.set("git_root_find", git_root_find)?;
    liz.set("git_is_ignored", git_is_ignored)?;

    Ok(())
}