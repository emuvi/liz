use rlua::UserData;

use crate::liz_debug::{dbg_call, dbg_reav, dbg_step, dbg_tell};
use crate::liz_forms;
use crate::liz_texts;

pub fn group_regex(regex: String) -> GroupBy {
    dbg_call!();
    dbg_reav!(GroupBy::Regex(regex));
}

pub fn group_white_space() -> GroupBy {
    dbg_call!();
    dbg_reav!(GroupBy::Imply(GroupImply::WhiteSpace));
}

pub fn group_punctuation() -> GroupBy {
    dbg_call!();
    dbg_reav!(GroupBy::Imply(GroupImply::Punctuation));
}

pub fn rig_group_all(forms: &mut Vec<String>, blocks: Vec<GroupBy>) -> usize {
    dbg_call!(forms, blocks);
    dbg_reav!(rig_group_on(forms, 0, liz_forms::kit_len(forms), blocks));
}

pub fn rig_group_on(
    forms: &mut Vec<String>,
    from: usize,
    till: usize,
    blocks: Vec<GroupBy>,
) -> usize {
    dbg_call!(forms, from, till, blocks);
    dbg_reav!(0);
}

#[derive(Debug, Clone, PartialEq)]
pub enum GroupBy {
    Equals(String),
    Likely(String),
    Regex(String),
    Imply(GroupImply),
}

impl UserData for GroupBy {}

impl GroupBy {
    pub fn get_trait(self) -> Box<dyn GroupTrait> {
        match self {
            GroupBy::Equals(equals) => Box::new(GroupEquals { equals }),
            GroupBy::Likely(likely) => Box::new(GroupLikely { likely }),
            GroupBy::Regex(regex) => Box::new(GroupRegex { regex }),
            GroupBy::Imply(imply) => match imply {
                GroupImply::WhiteSpace => Box::new(GroupWhiteSpace {}),
                GroupImply::Punctuation => Box::new(GroupPunctuation {}),
            },
        }
    }
}

#[derive(Debug)]
pub struct GroupEquals {
    equals: String,
}

impl GroupTrait for GroupEquals {
    fn checks(&self, term: &str) -> bool {
        liz_texts::is_equals(term, &self.equals)
    }
}

#[derive(Debug)]
pub struct GroupLikely {
    likely: String,
}

impl GroupTrait for GroupLikely {
    fn checks(&self, term: &str) -> bool {
        liz_texts::is_likely(term, &self.likely)
    }
}

#[derive(Debug)]
pub struct GroupRegex {
    regex: String,
}

impl GroupTrait for GroupRegex {
    fn checks(&self, term: &str) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GroupImply {
    WhiteSpace,
    Punctuation,
}

#[derive(Debug)]
pub struct GroupWhiteSpace {}

impl GroupTrait for GroupWhiteSpace {
    fn checks(&self, term: &str) -> bool {
        !term.chars().any(|ch| !ch.is_whitespace())
    }
}

#[derive(Debug)]
pub struct GroupPunctuation {}

impl GroupTrait for GroupPunctuation {
    fn checks(&self, term: &str) -> bool {
        !term.chars().any(|ch| !ch.is_ascii_punctuation())
    }
}

pub trait GroupTrait: std::fmt::Debug {
    fn checks(&self, term: &str) -> bool;
}

fn get_traits(blocks: Vec<GroupBy>) -> Vec<Box<dyn GroupTrait>> {
    dbg_call!(blocks);
    let mut result: Vec<Box<dyn GroupTrait>> = Vec::with_capacity(blocks.len());
    for block in blocks {
        result.push(block.get_trait());
    }
    dbg_reav!(result);
}
