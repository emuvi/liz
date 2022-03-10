use regex::Regex;
use rlua::UserData;

use crate::liz_debug::{dbg_call, dbg_reav, dbg_step, dbg_tell};
use crate::liz_forms;
use crate::LizError;

pub fn block_regex(regex: String) -> BlockBy {
    dbg_call!();
    dbg_reav!(BlockBy::Regex(regex));
}

pub fn block_white_space() -> BlockBy {
    dbg_call!();
    dbg_reav!(BlockBy::Imply(BlockImply::WhiteSpace));
}

pub fn block_punctuation() -> BlockBy {
    dbg_call!();
    dbg_reav!(BlockBy::Imply(BlockImply::Punctuation));
}

pub fn block_single_quotes() -> BlockBy {
    dbg_call!();
    dbg_reav!(BlockBy::Imply(BlockImply::SingleQuotes));
}

pub fn block_double_quotes() -> BlockBy {
    dbg_call!();
    dbg_reav!(BlockBy::Imply(BlockImply::DoubleQuotes));
}

pub fn rig_parse_all(forms: &mut Vec<String>, blocks: Vec<BlockBy>) -> Result<usize, LizError> {
    dbg_call!(forms, blocks);
    dbg_reav!(rig_parse_on(forms, 0, liz_forms::kit_len(forms), blocks));
}

pub fn rig_parse_on(
    forms: &mut Vec<String>,
    from: usize,
    till: usize,
    blocks: Vec<BlockBy>,
) -> Result<usize, LizError> {
    dbg_call!(forms, from, till, blocks);
    let range = liz_forms::kit_del_range(forms, from, till);
    dbg_step!(range);
    let parsers = get_parsers(blocks)?;
    let mut indexed_parsers: Vec<(usize, Box<dyn BlockTrait>)> =
        parsers.into_iter().enumerate().collect();
    dbg_step!(indexed_parsers);
    let mut helper = ParseHelper::new(range);
    let mut inside: i64 = -1;
    loop {
        dbg_tell!(inside);
        if inside < 0 {
            for (index, test_block) in &mut indexed_parsers {
                dbg_tell!(index, test_block);
                let opens_bound = test_block.opens(&mut helper);
                dbg_tell!(opens_bound);
                if opens_bound.checked {
                    if opens_bound.commits {
                        helper.commit_accrued();
                    }
                    helper.set_opened();
                    inside = *index as i64;
                    dbg_tell!(inside);
                    break;
                }
            }
        }
        helper.accrue_char_step();
        if inside >= 0 {
            let inside_block = &indexed_parsers[inside as usize].1;
            dbg_tell!(inside_block);
            let closes_bound = inside_block.closes(&mut helper);
            dbg_tell!(closes_bound);
            if closes_bound.checked {
                if closes_bound.commits {
                    helper.commit_accrued();
                }
                helper.set_closed();
                inside = -1;
                dbg_tell!(inside);
            }
        }
        if !helper.advance() {
            break;
        }
    }
    helper.commit_accrued();
    let results = helper.results;
    dbg_step!(results);
    let result = results.len();
    liz_forms::kit_add_range(forms, from, results);
    dbg_reav!(Ok(result));
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockBy {
    Regex(String),
    Imply(BlockImply),
}

impl UserData for BlockBy {}

impl BlockBy {
    pub fn get_trait(self) -> Result<Box<dyn BlockTrait>, LizError> {
        Ok(match self {
            BlockBy::Regex(regex) => Box::new(BlockRegex {
                regex: Regex::new(regex.as_ref())?,
            }),
            BlockBy::Imply(imply) => match imply {
                BlockImply::WhiteSpace => Box::new(BlockWhiteSpace {}),
                BlockImply::Punctuation => Box::new(BlockPunctuation {}),
                BlockImply::SingleQuotes => Box::new(BlockQuotation {
                    opener: '\'',
                    closer: '\'',
                    escape: '\\',
                }),
                BlockImply::DoubleQuotes => Box::new(BlockQuotation {
                    opener: '"',
                    closer: '"',
                    escape: '\\',
                }),
            },
        })
    }
}

#[derive(Debug)]
pub struct BlockRegex {
    regex: Regex,
}

impl BlockTrait for BlockRegex {
    fn opens(&self, helper: &mut ParseHelper) -> BlockBound {
        dbg_call!(helper);
        let checker = format!("{}{}", helper.get_accrued(), helper.get_char_step());
        dbg_step!(checker);
        let matcher = self.regex.find(&checker);
        dbg_step!(matcher);
        let mut checked = false;
        dbg_step!(checked);
        if let Some(matcher) = matcher {
            dbg_step!(matcher);
            checked = true;
            dbg_step!(checked);
            if matcher.start() > 0 {
                helper.commit_accrued_till(matcher.start());
            }
        }
        dbg_reav!(BlockBound {
            checked,
            commits: false,
        });
    }

    fn closes(&self, helper: &mut ParseHelper) -> BlockBound {
        dbg_call!(helper);
        let accrued = helper.get_accrued();
        dbg_step!(accrued);
        let checker = format!("{}{}", accrued, helper.get_char_next());
        dbg_step!(checker);
        let matcher = self.regex.find(&checker);
        dbg_step!(matcher);
        let mut checked = true;
        dbg_step!(checked);
        if let Some(matcher) = matcher {
            dbg_step!(matcher);
            if matcher.end() > accrued.len() {
                checked = false;
                dbg_step!(checked);
            }
        }
        dbg_reav!(BlockBound {
            checked,
            commits: true,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockImply {
    WhiteSpace,
    Punctuation,
    SingleQuotes,
    DoubleQuotes,
}

#[derive(Debug)]
pub struct BlockWhiteSpace {}

impl BlockTrait for BlockWhiteSpace {
    fn opens(&self, helper: &mut ParseHelper) -> BlockBound {
        dbg_call!(helper);
        dbg_reav!(BlockBound {
            checked: helper.get_char_step().is_whitespace(),
            commits: true,
        });
    }

    fn closes(&self, helper: &mut ParseHelper) -> BlockBound {
        dbg_call!(helper);
        dbg_reav!(BlockBound {
            checked: !helper.get_char_next().is_whitespace(),
            commits: true,
        });
    }
}

#[derive(Debug)]
pub struct BlockPunctuation {}

impl BlockTrait for BlockPunctuation {
    fn opens(&self, helper: &mut ParseHelper) -> BlockBound {
        dbg_call!(helper);
        dbg_reav!(BlockBound {
            checked: helper.get_char_step().is_ascii_punctuation(),
            commits: true,
        });
    }

    fn closes(&self, _: &mut ParseHelper) -> BlockBound {
        dbg_reav!(BlockBound {
            checked: true,
            commits: true,
        });
    }
}

#[derive(Debug)]
pub struct BlockQuotation {
    opener: char,
    closer: char,
    escape: char,
}

impl BlockTrait for BlockQuotation {
    fn opens(&self, helper: &mut ParseHelper) -> BlockBound {
        dbg_call!(helper);
        dbg_reav!(BlockBound {
            checked: helper.get_char_step() == self.opener,
            commits: true,
        })
    }

    fn closes(&self, helper: &mut ParseHelper) -> BlockBound {
        dbg_call!(helper);
        dbg_reav!(BlockBound {
            checked: !helper.is_step_on_opened()
                && helper.get_char_step() == self.closer
                && (helper.get_char_delta(-1) != self.escape
                    || (helper.get_char_delta(-1) == self.escape
                        && helper.get_char_delta(-2) == self.escape)),
            commits: true,
        });
    }
}

pub trait BlockTrait: std::fmt::Debug {
    fn opens(&self, helper: &mut ParseHelper) -> BlockBound;
    fn closes(&self, helper: &mut ParseHelper) -> BlockBound;
}

#[derive(Debug)]
pub struct BlockBound {
    checked: bool,
    commits: bool,
}

#[derive(Debug)]
pub struct ParseHelper {
    origins: Vec<char>,
    results: Vec<String>,
    accrued: String,
    char_step: i64,
    char_size: i64,
    opened_at: i64,
    step_accrued: bool,
}

impl ParseHelper {
    fn new(forms: Vec<String>) -> Self {
        dbg_call!(forms);
        let mut origins: Vec<char> = Vec::new();
        for form in forms {
            for ch in form.chars() {
                origins.push(ch);
            }
        }
        dbg_step!(origins);
        let char_size = origins.len() as i64;
        dbg_step!(char_size);
        dbg_reav!(ParseHelper {
            origins,
            results: Vec::new(),
            accrued: String::new(),
            char_step: 0,
            char_size: char_size,
            opened_at: -1,
            step_accrued: false,
        });
    }

    pub fn get_char_at(&self, place: i64) -> char {
        dbg_call!(place);
        if place >= 0 && place < self.char_size {
            dbg_reav!(self.origins[place as usize]);
        }
        dbg_reav!('\0');
    }

    pub fn get_char_past(&self) -> char {
        dbg_call!();
        dbg_reav!(self.get_char_at(self.char_step - 1));
    }

    pub fn get_char_step(&self) -> char {
        dbg_call!();
        dbg_reav!(self.get_char_at(self.char_step));
    }

    pub fn get_char_next(&self) -> char {
        dbg_call!();
        dbg_reav!(self.get_char_at(self.char_step + 1));
    }

    pub fn get_char_delta(&self, delta: i64) -> char {
        dbg_call!(delta);
        dbg_reav!(self.get_char_at(self.char_step + delta));
    }

    pub fn set_opened(&mut self) {
        dbg_call!();
        self.opened_at = self.char_step - self.accrued.len() as i64;
        dbg_step!(self.opened_at);
    }

    pub fn set_closed(&mut self) {
        dbg_call!();
        self.opened_at = -1;
        dbg_step!(self.opened_at);
    }

    pub fn set_opened_at(&mut self, place: i64) {
        dbg_call!(place);
        self.opened_at = place;
        dbg_step!(self.opened_at);
    }

    pub fn get_opened_at(&self) -> i64 {
        dbg_call!();
        dbg_reav!(self.opened_at);
    }

    pub fn is_step_on_opened(&self) -> bool {
        dbg_call!();
        dbg_reav!(self.char_step == self.opened_at);
    }

    pub fn has_accrued(&self) -> bool {
        dbg_call!();
        dbg_reav!(!self.accrued.is_empty());
    }

    pub fn get_accrued(&self) -> &str {
        dbg_call!();
        dbg_reav!(&self.accrued);
    }

    pub fn accrue_char_step(&mut self) {
        dbg_call!();
        if !self.step_accrued {
            self.accrued.push(self.get_char_step());
            dbg_step!(self.accrued);
            self.step_accrued = true;
            dbg_step!(self.step_accrued);
        }
    }

    pub fn commit_accrued(&mut self) {
        dbg_call!();
        if !self.accrued.is_empty() {
            let accrued = self.accrued.clone();
            dbg_step!(accrued);
            self.results.push(accrued);
            dbg_step!(self.results);
            self.accrued.clear();
        }
    }

    pub fn commit_accrued_till(&mut self, place: usize) {
        dbg_call!();
        if place < self.accrued.len() {
            let accrued_till = String::from(&self.accrued[0..place]);
            dbg_step!(accrued_till);
            self.results.push(accrued_till);
            dbg_step!(self.results);
            self.accrued = String::from(&self.accrued[place..]);
        }
    }

    pub fn advance(&mut self) -> bool {
        dbg_call!();
        if self.char_step < self.char_size - 1 {
            self.char_step += 1;
            dbg_step!(self.char_step);
            self.step_accrued = false;
            dbg_step!(self.step_accrued);
            dbg_reav!(true);
        } else {
            dbg_reav!(false);
        }
    }
}

fn get_parsers(blocks: Vec<BlockBy>) -> Result<Vec<Box<dyn BlockTrait>>, LizError> {
    dbg_call!(blocks);
    let mut result: Vec<Box<dyn BlockTrait>> = Vec::with_capacity(blocks.len());
    for block in blocks {
        result.push(block.get_trait()?);
    }
    dbg_reav!(Ok(result));
}
