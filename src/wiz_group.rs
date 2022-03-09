use rlua::{Context, Table};

use crate::liz_group::{self, GroupBy};
use crate::LizError;

pub fn inject_group<'a>(lane: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let group_regex = lane.create_function(|_, regex: String| Ok(liz_group::group_regex(regex)))?;

    let group_white_space = lane.create_function(|_, ()| Ok(liz_group::group_white_space()))?;

    let group_punctuation = lane.create_function(|_, ()| Ok(liz_group::group_punctuation()))?;

    let rig_group_all =
        lane.create_function_mut(|_, (mut forms, blocks): (Vec<String>, Vec<GroupBy>)| {
            liz_group::rig_group_all(&mut forms, blocks);
            Ok(forms)
        })?;

    let rig_group_on = lane.create_function_mut(
        |_, (mut forms, from, till, blocks): (Vec<String>, usize, usize, Vec<GroupBy>)| {
            liz_group::rig_group_on(&mut forms, from, till, blocks);
            Ok(forms)
        },
    )?;

    liz.set("group_regex", group_regex)?;
    liz.set("group_white_space", group_white_space)?;
    liz.set("group_punctuation", group_punctuation)?;
    liz.set("rig_group_all", rig_group_all)?;
    liz.set("rig_group_on", rig_group_on)?;

    Ok(())
}
