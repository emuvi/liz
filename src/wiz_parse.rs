use rlua::{Context, Table};

use crate::liz_parse::{self, BlockBy};
use crate::utils;
use crate::LizError;

pub fn inject_parse<'a>(lane: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let block_regex = lane.create_function(|_, regex: String| Ok(liz_parse::block_regex(regex)))?;

    let block_white_space = lane.create_function(|_, ()| Ok(liz_parse::block_white_space()))?;

    let block_alphabetic = lane.create_function(|_, ()| Ok(liz_parse::block_alphabetic()))?;

    let block_numeric = lane.create_function(|_, ()| Ok(liz_parse::block_numeric()))?;

    let block_alpha_numeric = lane.create_function(|_, ()| Ok(liz_parse::block_alpha_numeric()))?;

    let block_char_number = lane.create_function(|_, starter: String| {
        Ok(liz_parse::block_char_number(
            starter.chars().next().unwrap_or('\0'),
        ))
    })?;

    let block_punctuation = lane.create_function(|_, ()| Ok(liz_parse::block_punctuation()))?;

    let block_single_quotes = lane.create_function(|_, ()| Ok(liz_parse::block_single_quotes()))?;

    let block_double_quotes = lane.create_function(|_, ()| Ok(liz_parse::block_double_quotes()))?;

    let rig_parse_all =
        lane.create_function_mut(|_, (mut forms, blocks): (Vec<String>, Vec<BlockBy>)| {
            match utils::treat_error(liz_parse::rig_parse_all(&mut forms, &blocks)) {
                Ok(_) => Ok(forms),
                Err(err) => Err(err),
            }
        })?;

    let rig_parse_on = lane.create_function_mut(
        |_, (mut forms, from, till, blocks): (Vec<String>, usize, usize, Vec<BlockBy>)| {
            match utils::treat_error(liz_parse::rig_parse_on(&mut forms, from, till, &blocks)) {
                Ok(_) => Ok(forms),
                Err(err) => Err(err),
            }
        },
    )?;

    liz.set("block_regex", block_regex)?;
    liz.set("block_white_space", block_white_space)?;
    liz.set("block_alphabetic", block_alphabetic)?;
    liz.set("block_numeric", block_numeric)?;
    liz.set("block_alpha_numeric", block_alpha_numeric)?;
    liz.set("block_char_number", block_char_number)?;
    liz.set("block_punctuation", block_punctuation)?;
    liz.set("block_single_quotes", block_single_quotes)?;
    liz.set("block_double_quotes", block_double_quotes)?;
    liz.set("rig_parse_all", rig_parse_all)?;
    liz.set("rig_parse_on", rig_parse_on)?;

    Ok(())
}
