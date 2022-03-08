use rlua::{Context, Table};

use crate::liz_parse::{self, BlockedBy};
use crate::LizError;

pub fn inject_parse<'a>(lane: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let rig_whitespace = lane.create_function(|_, ()| Ok(liz_parse::rig_whitespace()))?;

    let rig_punctuation = lane.create_function(|_, ()| Ok(liz_parse::rig_punctuation()))?;

    let rig_parse_all =
        lane.create_function_mut(|_, (mut forms, blocks): (Vec<String>, Vec<BlockedBy>)| {
            liz_parse::rig_parse_all(&mut forms, blocks);
            Ok(forms)
        })?;

    let rig_parse_on = lane.create_function_mut(
        |_, (mut forms, from, till, blocks): (Vec<String>, usize, usize, Vec<BlockedBy>)| {
            liz_parse::rig_parse_on(&mut forms, from, till, blocks);
            Ok(forms)
        },
    )?;

    liz.set("rig_whitespace", rig_whitespace)?;
    liz.set("rig_punctuation", rig_punctuation)?;
    liz.set("rig_parse_all", rig_parse_all)?;
    liz.set("rig_parse_on", rig_parse_on)?;

    Ok(())
}
