use rlua::{Context, Table};

use crate::liz_parse;
use crate::liz_parse::BlockedBy;
    use crate::liz_parse::BlockParser;
use crate::LizError;

pub fn inject_parse<'a>(lane: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let rig_split_whitespace = lane.create_function_mut(|_, mut forms: Vec<String>| {
        liz_parse::rig_split_whitespace(&mut forms);
        Ok(forms)
    })?;

    let rig_split_whitespace_on =
        lane.create_function_mut(|_, (mut forms, from, till): (Vec<String>, usize, usize)| {
            liz_parse::rig_split_whitespace_on(&mut forms, from, till);
            Ok(forms)
        })?;

    let rig_split_punctuation = lane.create_function_mut(|_, mut forms: Vec<String>| {
        liz_parse::rig_split_punctuation(&mut forms);
        Ok(forms)
    })?;

    let rig_split_punctuation_on =
        lane.create_function_mut(|_, (mut forms, from, till): (Vec<String>, usize, usize)| {
            liz_parse::rig_split_punctuation_on(&mut forms, from, till);
            Ok(forms)
        })?;

    let rig_group_whitespace = lane.create_function_mut(|_, mut forms: Vec<String>| {
        liz_parse::rig_group_whitespace(&mut forms);
        Ok(forms)
    })?;

    let rig_group_whitespace_on =
        lane.create_function_mut(|_, (mut forms, from, till): (Vec<String>, usize, usize)| {
            liz_parse::rig_group_whitespace_on(&mut forms, from, till);
            Ok(forms)
        })?;

    let rig_group_punctuation = lane.create_function_mut(|_, mut forms: Vec<String>| {
        liz_parse::rig_group_punctuation(&mut forms);
        Ok(forms)
    })?;

    let rig_group_punctuation_on =
        lane.create_function_mut(|_, (mut forms, from, till): (Vec<String>, usize, usize)| {
            liz_parse::rig_group_punctuation_on(&mut forms, from, till);
            Ok(forms)
        })?;

    let rig_block_whitespace = lane.create_function(|_, ()| Ok (liz_parse::rig_block_whitespace()))?;

    let rig_block_punctuation = lane.create_function(|_, ()| Ok (liz_parse::rig_block_punctuation()))?;
    
    let rig_split =
        lane.create_function_mut(|_, (mut forms, from, till, blocks): (Vec<String>, usize, usize, Vec<BlockedBy>)| {
            let parsers = liz_parse::rig_blocks_parsers(blocks);
            liz_parse::rig_split(&mut forms, from, till, parsers);
            Ok(forms)
        })?;

    liz.set("rig_split_whitespace", rig_split_whitespace)?;
    liz.set("rig_split_whitespace_on", rig_split_whitespace_on)?;
    liz.set("rig_split_punctuation", rig_split_punctuation)?;
    liz.set("rig_split_punctuation_on", rig_split_punctuation_on)?;
    liz.set("rig_group_whitespace", rig_group_whitespace)?;
    liz.set("rig_group_whitespace_on", rig_group_whitespace_on)?;
    liz.set("rig_group_punctuation", rig_group_punctuation)?;
    liz.set("rig_group_punctuation_on", rig_group_punctuation_on)?;

    liz.set("rig_block_whitespace", rig_block_whitespace)?;
    liz.set("rig_block_punctuation", rig_block_punctuation)?;
    liz.set("rig_split", rig_split)?;

    Ok(())
}
