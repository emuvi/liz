use rlua::{Context, Table};

use crate::liz_group::{self, GroupIf, GroupPair};
use crate::utils;
use crate::LizError;

pub fn inject_group<'a>(lane: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let group_pair = lane.create_function(|_, (left, right): (GroupIf, GroupIf)| {
        Ok(liz_group::group_pair(left, right))
    })?;

    let group_equals = lane.create_function(|_, term: String| Ok(liz_group::group_equals(term)))?;

    let group_equals_not =
        lane.create_function(|_, term: String| Ok(liz_group::group_equals_not(term)))?;

    let group_likely = lane.create_function(|_, term: String| Ok(liz_group::group_likely(term)))?;

    let group_likely_not =
        lane.create_function(|_, term: String| Ok(liz_group::group_likely_not(term)))?;

    let group_regex =
        lane.create_function(|_, phrase: String| Ok(liz_group::group_regex(phrase)))?;

    let group_regex_not =
        lane.create_function(|_, phrase: String| Ok(liz_group::group_regex_not(phrase)))?;

    let group_any = lane.create_function(|_, ()| Ok(liz_group::group_any()))?;

    let group_any_not = lane.create_function(|_, ()| Ok(liz_group::group_any_not()))?;

    let group_white_space = lane.create_function(|_, ()| Ok(liz_group::group_white_space()))?;

    let group_white_space_not =
        lane.create_function(|_, ()| Ok(liz_group::group_white_space_not()))?;

    let group_punctuation = lane.create_function(|_, ()| Ok(liz_group::group_punctuation()))?;

    let group_punctuation_not =
        lane.create_function(|_, ()| Ok(liz_group::group_punctuation_not()))?;

    let rig_group_all = lane.create_function_mut(
        |_, (mut forms, groups, recursive): (Vec<String>, Vec<GroupPair>, bool)| {
            let groupers = match utils::treat_error(liz_group::get_groupers(groups)) {
                Ok(groupers) => groupers,
                Err(err) => (return Err(err)),
            };
            match utils::treat_error(liz_group::rig_group_all(&mut forms, &groupers, recursive)) {
                Ok(_) => Ok(forms),
                Err(err) => Err(err),
            }
        },
    )?;

    let rig_group_on = lane.create_function_mut(
        |_,
         (mut forms, from, till, groups, recursive): (
            Vec<String>,
            usize,
            usize,
            Vec<GroupPair>,
            bool,
        )| {
            let groupers = match utils::treat_error(liz_group::get_groupers(groups)) {
                Ok(groupers) => groupers,
                Err(err) => (return Err(err)),
            };
            match utils::treat_error(liz_group::rig_group_on(
                &mut forms, from, till, &groupers, recursive,
            )) {
                Ok(_) => Ok(forms),
                Err(err) => Err(err),
            }
        },
    )?;

    liz.set("group_pair", group_pair)?;
    liz.set("group_equals", group_equals)?;
    liz.set("group_equals_not", group_equals_not)?;
    liz.set("group_likely", group_likely)?;
    liz.set("group_likely_not", group_likely_not)?;
    liz.set("group_regex", group_regex)?;
    liz.set("group_regex_not", group_regex_not)?;
    liz.set("group_any", group_any)?;
    liz.set("group_any_not", group_any_not)?;
    liz.set("group_white_space", group_white_space)?;
    liz.set("group_white_space_not", group_white_space_not)?;
    liz.set("group_punctuation", group_punctuation)?;
    liz.set("group_punctuation_not", group_punctuation_not)?;
    liz.set("rig_group_all", rig_group_all)?;
    liz.set("rig_group_on", rig_group_on)?;

    Ok(())
}
