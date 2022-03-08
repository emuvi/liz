use rlua::{Context, Table};

use crate::liz_parse::{self, BlockBy};
use crate::LizError;

pub fn inject_parse<'a>(lane: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let rig_white_space = lane.create_function(|_, ()| Ok(liz_parse::rig_white_space()))?;

    let rig_punctuation = lane.create_function(|_, ()| Ok(liz_parse::rig_punctuation()))?;

    let rig_single_quotes = lane.create_function(|_, ()| Ok(liz_parse::rig_single_quotes()))?;

    let rig_double_quotes = lane.create_function(|_, ()| Ok(liz_parse::rig_double_quotes()))?;

    let rig_parse_all =
        lane.create_function_mut(|_, (mut forms, blocks): (Vec<String>, Vec<BlockBy>)| {
            liz_parse::rig_parse_all(&mut forms, blocks);
            Ok(forms)
        })?;

    let rig_parse_on = lane.create_function_mut(
        |_, (mut forms, from, till, blocks): (Vec<String>, usize, usize, Vec<BlockBy>)| {
            liz_parse::rig_parse_on(&mut forms, from, till, blocks);
            Ok(forms)
        },
    )?;

    liz.set("rig_white_space", rig_white_space)?;
    liz.set("rig_punctuation", rig_punctuation)?;
    liz.set("rig_single_quotes", rig_single_quotes)?;
    liz.set("rig_double_quotes", rig_double_quotes)?;
    liz.set("rig_parse_all", rig_parse_all)?;
    liz.set("rig_parse_on", rig_parse_on)?;

    Ok(())
}
