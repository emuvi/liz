use rlua::UserData;

use crate::liz_debug::{dbg_call, dbg_reav, dbg_step, dbg_tell};
use crate::liz_forms;

pub fn rig_white_space() -> BlockBy {
    BlockBy::WhiteSpace
}

pub fn rig_punctuation() -> BlockBy {
    BlockBy::Punctuation
}

pub fn rig_single_quotes() -> BlockBy {
    BlockBy::SingleQuotes
}

pub fn rig_double_quotes() -> BlockBy {
    BlockBy::DoubleQuotes
}

pub fn rig_parse_all(forms: &mut Vec<String>, blocks: Vec<BlockBy>) -> usize {
    rig_parse_on(forms, 0, liz_forms::kit_len(forms), blocks)
}

pub fn rig_parse_on(
    forms: &mut Vec<String>,
    from: usize,
    till: usize,
    blocks: Vec<BlockBy>,
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
                    if opens_bound.include {
                        helper.commit_accrued();
                        if !already_accrued_now {
                            helper.accrue_char_step();
                        }
                    } else {
                        if !already_accrued_now {
                            helper.accrue_char_step();
                        }
                        helper.commit_accrued();
                    }
                    already_accrued_now = true;
                    helper.set_opened();
                    inside = index as i64;
                }
            }
        }
        if inside >= 0 {
            let inside_block = &parsers[inside as usize];
            let closes_bound = inside_block.closes(&helper);
            if closes_bound.checked {
                if closes_bound.include {
                    if !already_accrued_now {
                        helper.accrue_char_step();
                    }
                    helper.commit_accrued();
                } else {
                    helper.commit_accrued();
                    if !already_accrued_now {
                        helper.accrue_char_step();
                    }
                }
                already_accrued_now = true;
                helper.set_closed();
                inside = -1;
            }
        }
        if !already_accrued_now {
            helper.accrue_char_step();
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

fn get_parsers(blocks: Vec<BlockBy>) -> Vec<&'static dyn BlockParser> {
    blocks.iter().map(|block| block.get_parser()).collect()
}

#[derive(Clone, PartialEq)]
pub enum BlockBy {
    WhiteSpace,
    Punctuation,
    SingleQuotes,
    DoubleQuotes,
}

impl BlockBy {
    pub fn get_parser(&self) -> &'static dyn BlockParser {
        match &self {
            BlockBy::WhiteSpace => &BLOCK_WHITE_SPACE,
            BlockBy::Punctuation => &BLOCK_PUNCTUATION,
            BlockBy::SingleQuotes => &BLOCK_SINGLE_QUOTES,
            BlockBy::DoubleQuotes => &BLOCK_DOUBLE_QUOTES,
        }
    }
}

impl UserData for BlockBy {}

pub static BLOCK_WHITE_SPACE: BlockWhiteSpace = BlockWhiteSpace {};

pub static BLOCK_PUNCTUATION: BlockPunctuation = BlockPunctuation {};

pub static BLOCK_SINGLE_QUOTES: BlockQuotation = BlockQuotation {
    opener: '\'',
    closer: '\'',
    escape: '\\',
};

pub static BLOCK_DOUBLE_QUOTES: BlockQuotation = BlockQuotation {
    opener: '"',
    closer: '"',
    escape: '\\',
};

pub struct BlockWhiteSpace {}

impl BlockParser for BlockWhiteSpace {
    fn opens(&self, helper: &ParserHelper) -> BlockBound {
        BlockBound {
            checked: helper.get_char_step().is_whitespace(),
            include: true,
        }
    }
    fn closes(&self, helper: &ParserHelper) -> BlockBound {
        BlockBound {
            checked: !helper.get_char_step().is_whitespace(),
            include: false,
        }
    }
}

pub struct BlockPunctuation {}

impl BlockParser for BlockPunctuation {
    fn opens(&self, helper: &ParserHelper) -> BlockBound {
        BlockBound {
            checked: helper.get_char_step().is_ascii_punctuation(),
            include: true,
        }
    }
    fn closes(&self, _: &ParserHelper) -> BlockBound {
        BlockBound {
            checked: true,
            include: false,
        }
    }
}

pub struct BlockQuotation {
    opener: char,
    closer: char,
    escape: char,
}

impl BlockParser for BlockQuotation {
    fn opens(&self, helper: &ParserHelper) -> BlockBound {
        BlockBound {
            checked: helper.get_char_step() == self.opener,
            include: true,
        }
    }

    fn closes(&self, helper: &ParserHelper) -> BlockBound {
        BlockBound {
            checked: !helper.is_step_on_opened()
                && helper.get_char_step() == self.closer
                && (helper.get_char_delta(-1) != self.escape
                    || (helper.get_char_delta(-1) == self.escape
                        && helper.get_char_delta(-2) == self.escape)),
            include: true,
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
    char_step: i64,
    char_size: i64,
    opened_at: i64,
}

impl ParserHelper {
    fn new(forms: Vec<String>) -> Self {
        let mut origins: Vec<char> = Vec::new();
        for form in forms {
            for ch in form.chars() {
                origins.push(ch);
            }
        }
        let char_size = origins.len() as i64;
        Self {
            origins,
            results: Vec::new(),
            accrued: String::new(),
            char_step: 0,
            char_size: char_size,
            opened_at: -1,
        }
    }

    pub fn advance(&mut self) -> bool {
        if self.char_step < self.char_size - 1 {
            self.char_step += 1;
            true
        } else {
            false
        }
    }

    pub fn get_char_at(&self, place: i64) -> char {
        if place >= 0 && place < self.char_size {
            return self.origins[place as usize];
        }
        '\0'
    }

    pub fn get_char_step(&self) -> char {
        self.get_char_at(self.char_step)
    }

    pub fn get_char_delta(&self, delta: i64) -> char {
        self.get_char_at(self.char_step + delta)
    }

    pub fn set_opened(&mut self) {
        self.opened_at = self.char_step;
    }

    pub fn set_closed(&mut self) {
        self.opened_at = -1;
    }

    pub fn set_opened_at(&mut self, place: i64) {
        self.opened_at = place;
    }

    pub fn get_opened_at(&self) -> i64 {
        self.opened_at
    }

    pub fn is_step_on_opened(&self) -> bool {
        self.char_step == self.opened_at
    }

    pub fn accrue_char_step(&mut self) {
        self.accrued.push(self.get_char_step());
    }

    pub fn commit_accrued(&mut self) {
        if !self.accrued.is_empty() {
            let accrued = self.accrued.clone();
            dbg_tell!(accrued);
            self.results.push(accrued);
            self.accrued.clear();
        }
    }
}
