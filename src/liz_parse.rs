use rlua::UserData;

use crate::liz_debug::{dbg_call, dbg_reav, dbg_step, dbg_tell};
use crate::liz_forms;

pub fn rig_whitespace() -> BlockedBy {
    BlockedBy::WhiteSpace
}

pub fn rig_punctuation() -> BlockedBy {
    BlockedBy::Punctuation
}

pub fn rig_parse_all(
    forms: &mut Vec<String>,
    blocks: Vec<BlockedBy>
) -> usize {
    rig_parse_on(forms, 0, liz_forms::kit_len(forms), blocks)
}

pub fn rig_parse_on(
    forms: &mut Vec<String>,
    from: usize,
    till: usize,
    blocks: Vec<BlockedBy>
) -> usize {
    dbg_call!(forms, from, till);
    let range = liz_forms::kit_del_range(forms, from, till);
    dbg_step!(range);
    let parsers = get_parsers(blocks);
    let mut helper = ParserHelper::new(range);
    let mut inside: i64 = -1;
    loop {
        let mut already_accrued_now = false;
        if inside < 0 {
            for (index, test_block) in parsers.iter().enumerate() {
                let opens_bound = test_block.opens(&helper);
                if opens_bound.checked {
                    if !opens_bound.include {
                        helper.accrue_char_now();
                        already_accrued_now = true;
                    }
                    helper.commit_accrued();
                    inside = index as i64;
                }
            }
        }
        if inside >= 0 {
            let inside_block = &parsers[inside as usize];
            let closes_bound = inside_block.closes(&helper);
            if closes_bound.checked {
                if closes_bound.include {
                    helper.accrue_char_now();
                    already_accrued_now = true;
                }
                helper.commit_accrued();
                inside = -1;
            }
        }
        if !already_accrued_now {
            helper.accrue_char_now();
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
    dbg_reav!(result);
}

fn get_parsers(blocks: Vec<BlockedBy>) -> Vec<&'static dyn BlockParser> {
    blocks.iter().map(|block| block.get_parser()).collect()
}

#[derive(Clone, PartialEq)]
pub enum BlockedBy {
    WhiteSpace,
    Punctuation,
}

impl BlockedBy {
    pub fn get_parser(&self) -> &'static dyn BlockParser {
        match &self {
            BlockedBy::WhiteSpace => &BLOCK_WHITE_SPACE,
            BlockedBy::Punctuation => &BLOCK_PUNCTUATION,
        }
    }
}

impl UserData for BlockedBy {}

pub static BLOCK_WHITE_SPACE: BlockWhiteSpace = BlockWhiteSpace {};
pub static BLOCK_PUNCTUATION: BlockPunctuation = BlockPunctuation {};

pub struct BlockWhiteSpace {}

impl BlockParser for BlockWhiteSpace {
    fn opens(&self, helper: &ParserHelper) -> BlockBound {
        BlockBound {
            checked: helper.get_char_now().is_whitespace(),
            include: true,
        }
    }
    fn closes(&self, helper: &ParserHelper) -> BlockBound {
        BlockBound {
            checked: !helper.get_char_now().is_whitespace(),
            include: false,
        }
    }
}

pub struct BlockPunctuation {}

impl BlockParser for BlockPunctuation {
    fn opens(&self, helper: &ParserHelper) -> BlockBound {
        BlockBound {
            checked: helper.get_char_now().is_ascii_punctuation(),
            include: true,
        }
    }
    fn closes(&self, helper: &ParserHelper) -> BlockBound {
        BlockBound {
            checked: helper.get_char_now().is_ascii_punctuation(),
            include: false,
        }
    }
}

pub struct BlockBound {
    pub checked: bool,
    pub include: bool,
}

pub trait BlockParser {
    fn opens(&self, helper: &ParserHelper) -> BlockBound;
    fn closes(&self, helper: &ParserHelper) -> BlockBound;
}

pub struct ParserHelper {
    origins: Vec<char>,
    results: Vec<String>,
    accrued: String,
    char_now: i64,
    chars_size: i64,
}

impl ParserHelper {
    fn new(forms: Vec<String>) -> Self {
        let mut origins: Vec<char> = Vec::new();
        for form in forms {
            for ch in form.chars() {
                origins.push(ch);
            }
        }
        let chars_size = origins.len() as i64;
        Self {
            origins,
            results: Vec::new(),
            accrued: String::new(),
            char_now: 0,
            chars_size,
        }
    }

    fn advance(&mut self) -> bool {
        if self.char_now < self.chars_size - 1 {
            self.char_now += 1;
            true
        } else {
            false
        }
    }

    fn get_char(&self, at: i64) -> char {
        if at >= 0 && at < self.chars_size {
            return self.origins[at as usize];
        }
        '\0'
    }

    fn get_char_now(&self) -> char {
        self.get_char(self.char_now)
    }

    fn accrue_char_now(&mut self) {
        self.accrued.push(self.get_char_now());
    }

    fn commit_accrued(&mut self) {
        if !self.accrued.is_empty() {
            let accrued = self.accrued.clone();
            dbg_tell!(accrued);
            self.results.push(accrued);
            self.accrued.clear();
        }
    }
}
