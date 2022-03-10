use regex::Regex;
use rlua::UserData;

use crate::liz_debug::{dbg_call, dbg_reav, dbg_step, dbg_tell};
use crate::liz_forms;
use crate::liz_logic::{self, Sense};
use crate::liz_texts;
use crate::LizError;

pub fn group_pair(left: GroupIf, right: GroupIf) -> GroupPair {
    dbg_call!(left, right);
    dbg_reav!(GroupPair { left, right });
}

pub fn group_equals(term: String) -> GroupIf {
    dbg_call!();
    dbg_reav!(GroupIf::Equals(Sense::Same, term));
}

pub fn group_equals_not(term: String) -> GroupIf {
    dbg_call!();
    dbg_reav!(GroupIf::Equals(Sense::Swap, term));
}

pub fn group_likely(term: String) -> GroupIf {
    dbg_call!();
    dbg_reav!(GroupIf::Likely(Sense::Same, term));
}

pub fn group_likely_not(term: String) -> GroupIf {
    dbg_call!();
    dbg_reav!(GroupIf::Likely(Sense::Swap, term));
}

pub fn group_regex(phrase: String) -> GroupIf {
    dbg_call!();
    dbg_reav!(GroupIf::Regex(Sense::Same, phrase));
}

pub fn group_regex_not(regex: String) -> GroupIf {
    dbg_call!();
    dbg_reav!(GroupIf::Regex(Sense::Swap, regex));
}

pub fn group_any() -> GroupIf {
    dbg_call!();
    dbg_reav!(GroupIf::Imply(Sense::Same, GroupImply::Any));
}

pub fn group_any_not() -> GroupIf {
    dbg_call!();
    dbg_reav!(GroupIf::Imply(Sense::Swap, GroupImply::Any));
}

pub fn group_white_space() -> GroupIf {
    dbg_call!();
    dbg_reav!(GroupIf::Imply(Sense::Same, GroupImply::WhiteSpace));
}

pub fn group_white_space_not() -> GroupIf {
    dbg_call!();
    dbg_reav!(GroupIf::Imply(Sense::Swap, GroupImply::WhiteSpace));
}

pub fn group_punctuation() -> GroupIf {
    dbg_call!();
    dbg_reav!(GroupIf::Imply(Sense::Same, GroupImply::Punctuation));
}

pub fn group_punctuation_not() -> GroupIf {
    dbg_call!();
    dbg_reav!(GroupIf::Imply(Sense::Swap, GroupImply::Punctuation));
}

pub fn rig_group_all(
    forms: &mut Vec<String>,
    groups: Vec<GroupPair>,
    recursive: bool,
) -> Result<usize, LizError> {
    dbg_call!(forms, groups);
    dbg_reav!(rig_group_on(
        forms,
        0,
        liz_forms::kit_len(forms),
        groups,
        recursive
    ));
}

pub fn rig_group_on(
    forms: &mut Vec<String>,
    from: usize,
    till: usize,
    groups: Vec<GroupPair>,
    recursive: bool,
) -> Result<usize, LizError> {
    dbg_call!(forms, from, till, groups, recursive);
    let range = liz_forms::kit_del_range(forms, from, till);
    dbg_step!(range);
    let groupers = get_groupers(groups)?;
    dbg_step!(groupers);
    let mut helper = GroupHelper::new(range);
    dbg_step!(helper);
    while let Some(term) = helper.advance() {
        dbg_tell!(term);
        if !helper.has_accrued() {
            helper.accrue_term(term);
        } else {
            let mut should_group = false;
            let left = helper.get_accrued();
            let right = &term;
            for (left_test, right_test) in &groupers {
                let left_check = left_test.checks(left);
                let right_check = right_test.checks(right);
                if left_check && right_check {
                    should_group = true;
                    break;
                }
            }
            if should_group {
                helper.accrue_term(term);
                if !recursive {
                    helper.commit_accrued();
                }
            } else {
                helper.commit_accrued();
                helper.accrue_term(term);
            }
        }
    }
    helper.commit_accrued();
    let results = helper.results;
    let result = results.len();
    liz_forms::kit_add_range(forms, from, results);
    dbg_reav!(Ok(result));
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupPair {
    left: GroupIf,
    right: GroupIf,
}

impl UserData for GroupPair {}

#[derive(Debug, Clone, PartialEq)]
pub enum GroupIf {
    Equals(Sense, String),
    Likely(Sense, String),
    Regex(Sense, String),
    Imply(Sense, GroupImply),
}

impl UserData for GroupIf {}

impl GroupIf {
    pub fn get_trait(self) -> Result<Box<dyn GroupTrait>, LizError> {
        Ok(match self {
            GroupIf::Equals(sense, equals) => Box::new(GroupEquals { sense, equals }),
            GroupIf::Likely(sense, likely) => Box::new(GroupLikely { sense, likely }),
            GroupIf::Regex(sense, phrase) => Box::new(GroupRegex {
                sense,
                regex: Regex::new(phrase.as_ref())?,
            }),
            GroupIf::Imply(sense, imply) => match imply {
                GroupImply::Any => Box::new(GroupAny { sense }),
                GroupImply::WhiteSpace => Box::new(GroupWhiteSpace { sense }),
                GroupImply::Punctuation => Box::new(GroupPunctuation { sense }),
            },
        })
    }
}

#[derive(Debug)]
pub struct GroupEquals {
    sense: Sense,
    equals: String,
}

impl GroupTrait for GroupEquals {
    fn checks(&self, term: &str) -> bool {
        liz_logic::sense_apply(self.sense, liz_texts::is_equals(term, &self.equals))
    }
}

#[derive(Debug)]
pub struct GroupLikely {
    sense: Sense,
    likely: String,
}

impl GroupTrait for GroupLikely {
    fn checks(&self, term: &str) -> bool {
        liz_logic::sense_apply(self.sense, liz_texts::is_likely(term, &self.likely))
    }
}

#[derive(Debug)]
pub struct GroupRegex {
    sense: Sense,
    regex: Regex,
}

impl GroupTrait for GroupRegex {
    fn checks(&self, term: &str) -> bool {
        liz_logic::sense_apply(self.sense, self.regex.is_match(term))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GroupImply {
    Any,
    WhiteSpace,
    Punctuation,
}

#[derive(Debug)]
pub struct GroupAny {
    sense: Sense,
}

impl GroupTrait for GroupAny {
    fn checks(&self, _: &str) -> bool {
        liz_logic::sense_apply(self.sense, true)
    }
}

#[derive(Debug)]
pub struct GroupWhiteSpace {
    sense: Sense,
}

impl GroupTrait for GroupWhiteSpace {
    fn checks(&self, term: &str) -> bool {
        liz_logic::sense_apply(self.sense, !term.chars().any(|ch| !ch.is_whitespace()))
    }
}

#[derive(Debug)]
pub struct GroupPunctuation {
    sense: Sense,
}

impl GroupTrait for GroupPunctuation {
    fn checks(&self, term: &str) -> bool {
        liz_logic::sense_apply(
            self.sense,
            !term.chars().any(|ch| !ch.is_ascii_punctuation()),
        )
    }
}

pub trait GroupTrait: std::fmt::Debug {
    fn checks(&self, term: &str) -> bool;
}

#[derive(Debug)]
pub struct GroupHelper {
    origins: Vec<String>,
    results: Vec<String>,
    accrued: Option<String>,
}

impl GroupHelper {
    fn new(origins: Vec<String>) -> Self {
        dbg_call!(origins);
        dbg_reav!(Self {
            origins,
            results: Vec::new(),
            accrued: None,
        });
    }

    pub fn has_accrued(&self) -> bool {
        dbg_call!();
        dbg_reav!(self.accrued.is_some());
    }

    pub fn get_accrued(&self) -> &str {
        dbg_call!();
        dbg_reav!(if let Some(ref accrued) = self.accrued {
            accrued
        } else {
            ""
        });
    }

    pub fn accrue_term(&mut self, term: String) {
        dbg_call!(term);
        if self.accrued.is_none() {
            self.accrued = Some(term);
        } else if let Some(ref mut existing) = self.accrued {
            existing.push_str(&term);
        }
        dbg_step!(self.accrued);
    }

    pub fn commit_accrued(&mut self) {
        dbg_call!();
        if let Some(ref accrued) = self.accrued {
            dbg_step!(accrued);
            self.results.push(accrued.clone());
            dbg_step!(self.results);
            self.accrued = None;
        }
    }

    pub fn advance(&mut self) -> Option<String> {
        dbg_call!();
        dbg_reav!(if !self.origins.is_empty() {
            Some(self.origins.remove(0))
        } else {
            None
        });
    }
}

fn get_groupers(
    groups: Vec<GroupPair>,
) -> Result<Vec<(Box<dyn GroupTrait>, Box<dyn GroupTrait>)>, LizError> {
    dbg_call!(groups);
    let mut result: Vec<(Box<dyn GroupTrait>, Box<dyn GroupTrait>)> =
        Vec::with_capacity(groups.len());
    for group in groups {
        result.push((group.left.get_trait()?, group.right.get_trait()?));
    }
    dbg_reav!(Ok(result));
}
