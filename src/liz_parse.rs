use rlua::UserData;

use crate::liz_debug::{dbg_call, dbg_reav, dbg_step, dbg_tell};
use crate::liz_forms;

pub fn rig_split_whitespace(forms: &mut Vec<String>) -> usize {
    dbg_call!(forms);
    dbg_reav!(rig_split_whitespace_on(forms, 0, liz_forms::kit_len(forms)));
}

pub fn rig_split_whitespace_on(forms: &mut Vec<String>, from: usize, till: usize) -> usize {
    dbg_call!(forms, from, till);
    dbg_reav!(rig_split_near_ask_on(forms, from, till, |ch| ch.is_whitespace()));
}

pub fn rig_split_punctuation(forms: &mut Vec<String>) -> usize {
    dbg_call!(forms);
    dbg_reav!(rig_split_punctuation_on(
        forms,
        0,
        liz_forms::kit_len(forms)
    ));
}

pub fn rig_split_punctuation_on(forms: &mut Vec<String>, from: usize, till: usize) -> usize {
    dbg_call!(forms, from, till);
    dbg_reav!(rig_split_each_ask_on(forms, from, till, |ch| ch.is_ascii_punctuation()));
}

pub fn rig_group_whitespace(forms: &mut Vec<String>) -> usize {
    dbg_call!(forms);
    dbg_reav!(rig_group_whitespace_on(forms, 0, liz_forms::kit_len(forms)));
}

pub fn rig_group_whitespace_on(forms: &mut Vec<String>, from: usize, till: usize) -> usize {
    dbg_call!(forms, from, till);
    dbg_reav!(rig_group_near_ask_on(forms, from, till, |ch| ch.is_whitespace()));
}

pub fn rig_group_punctuation(forms: &mut Vec<String>) -> usize {
    dbg_call!(forms);
    dbg_reav!(rig_group_punctuation_on(
        forms,
        0,
        liz_forms::kit_len(forms)
    ));
}

pub fn rig_group_punctuation_on(forms: &mut Vec<String>, from: usize, till: usize) -> usize {
    dbg_call!(forms, from, till);
    dbg_reav!(rig_group_each_ask_on(forms, from, till, |ch| ch.is_ascii_punctuation()));
}

pub fn rig_split_near_ask_on<F: Fn(char) -> bool>(
    forms: &mut Vec<String>,
    from: usize,
    till: usize,
    ask: F,
) -> usize {
    dbg_call!(forms, from, till);
    let range = liz_forms::kit_del_range(forms, from, till);
    dbg_step!(range);
    let mut helps = OldHelper::new();
    let mut state = false;
    for form in range {
        dbg_tell!(form);
        helps.commit_accrued();
        for ch in form.chars() {
            if ask(ch) != state {
                helps.commit_accrued();
                state = !state;
            }
            helps.accrue_char(ch);
        }
    }
    helps.commit_accrued();
    let results = helps.results;
    dbg_step!(results);
    let result = results.len();
    liz_forms::kit_add_range(forms, from, results);
    dbg_reav!(result);
}

pub fn rig_split_each_ask_on<F: Fn(char) -> bool>(
    forms: &mut Vec<String>,
    from: usize,
    till: usize,
    ask: F,
) -> usize {
    dbg_call!(forms, from, till);
    let range = liz_forms::kit_del_range(forms, from, till);
    dbg_step!(range);
    let mut helps = OldHelper::new();
    for form in range {
        dbg_tell!(form);
        helps.commit_accrued();
        for ch in form.chars() {
            if ask(ch) {
                helps.commit_accrued();
                helps.got_form(String::from(ch));
            } else {
                helps.accrue_char(ch);
            }
        }
    }
    helps.commit_accrued();
    let results = helps.results;
    dbg_step!(results);
    let result = results.len();
    liz_forms::kit_add_range(forms, from, results);
    dbg_reav!(result);
}

pub fn rig_group_near_ask_on<F: Fn(char) -> bool>(
    forms: &mut Vec<String>,
    from: usize,
    till: usize,
    ask: F,
) -> usize {
    dbg_call!(forms, from, till);
    let mut range = liz_forms::kit_del_range(forms, from, till);
    dbg_step!(range);
    let mut results = Vec::new();
    loop {
        if range.is_empty() {
            break;
        }
        let side_a = if !results.is_empty() {
            results.pop().unwrap()
        } else {
            range.remove(0)
        };
        dbg_tell!(side_a);
        if range.is_empty() {
            results.push(side_a);
        } else {
            let side_b = range.remove(0);
            dbg_tell!(side_b);
            let side_a_last = side_a.chars().last();
            dbg_tell!(side_a_last);
            let side_b_first = side_b.chars().next();
            dbg_tell!(side_b_first);
            let mut should_group = false;
            if let Some(side_a_last) = side_a_last {
                if let Some(side_b_first) = side_b_first {
                    if ask(side_a_last) != ask(side_b_first) {
                        should_group = true;
                    }
                }
            }
            dbg_tell!(should_group);
            if should_group {
                let grouped = side_a + &side_b;
                dbg_tell!(grouped);
                results.push(grouped);
            } else {
                results.push(side_a);
                results.push(side_b);
            }
        }
    }
    dbg_step!(results);
    let result = results.len();
    liz_forms::kit_add_range(forms, from, results);
    dbg_reav!(result);
}

pub fn rig_group_each_ask_on<F: Fn(char) -> bool>(
    forms: &mut Vec<String>,
    from: usize,
    till: usize,
    ask: F,
) -> usize {
    dbg_call!(forms, from, till);
    let mut range = liz_forms::kit_del_range(forms, from, till);
    dbg_step!(range);
    let mut results = Vec::new();
    let mut last_should_group = false;
    loop {
        if range.is_empty() {
            break;
        }
        let side_a = if !results.is_empty() {
            results.pop().unwrap()
        } else {
            range.remove(0)
        };
        dbg_tell!(side_a);
        if range.is_empty() {
            results.push(side_a);
        } else {
            let side_b = range.remove(0);
            dbg_tell!(side_b);
            let mut should_group = false;
            if last_should_group {
                should_group = true;
                last_should_group = false;
            }
            if let Some((last_index, side_b_char)) = side_b.chars().enumerate().last() {
                if last_index == 0 && ask(side_b_char) {
                    should_group = true;
                    last_should_group = true;
                }
            }
            dbg_tell!(should_group);
            if should_group {
                let grouped = side_a + &side_b;
                dbg_tell!(grouped);
                results.push(grouped);
            } else {
                results.push(side_a);
                results.push(side_b);
            }
        }
    }
    dbg_step!(results);
    let result = results.len();
    liz_forms::kit_add_range(forms, from, results);
    dbg_reav!(result);
}

struct OldHelper {
    results: Vec<String>,
    accrued: String,
}

impl OldHelper {
    fn new() -> Self {
        Self {
            results: Vec::new(),
            accrued: String::new(),
        }
    }

    fn accrue_char(&mut self, ch: char) {
        self.accrued.push(ch);
    }

    fn commit_accrued(&mut self) {
        if !self.accrued.is_empty() {
            let accrued = self.accrued.clone();
            dbg_tell!(accrued);
            self.results.push(accrued);
            self.accrued.clear();
        }
    }

    fn got_form(&mut self, from: String) {
        dbg_tell!(from);
        self.results.push(from);
    }
}

#[derive(Clone, PartialEq)]
pub enum BlockedBy {
    WhiteSpace,
    Punctuation
}

pub fn rig_block_whitespace() -> BlockedBy {
    BlockedBy::WhiteSpace
}

pub fn rig_block_punctuation() -> BlockedBy {
    BlockedBy::Punctuation
}

impl BlockedBy {
    pub fn get_parser(&self) -> &dyn BlockParser {
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

pub fn rig_split(
    forms: &mut Vec<String>,
    from: usize,
    till: usize,
    blocks: Vec<Box<&dyn BlockParser>>,
) -> usize {
    dbg_call!(forms, from, till);
    let range = liz_forms::kit_del_range(forms, from, till);
    dbg_step!(range);
    let mut helper = ParserHelper::new(range);
    let mut inside: i64 = -1;
    loop {
        let mut already_accrued_now = false;
        if inside < 0 {
            for (index, test_block) in blocks.iter().enumerate() {
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
            let inside_block = &blocks[inside as usize];
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
    let results = helper.results;
    dbg_step!(results);
    let result = results.len();
    liz_forms::kit_add_range(forms, from, results);
    dbg_reav!(result);
}

pub struct ParserHelper {
    origins: Vec<Vec<char>>,
    results: Vec<String>,
    accrued: String,
    char_now: usize,
    chars_size: usize,
}

impl ParserHelper {
    fn new(forms: Vec<String>) -> Self {
        let mut origins: Vec<Vec<char>> = Vec::new();
        let mut chars_size = 0;
        for form in forms {
            let form_chars: Vec<char> = form.chars().collect();
            chars_size += form_chars.len();
            origins.push(form_chars);
        }
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

    fn get_char(&self, at: usize) -> char {
        let mut total_size = 0;
        for origin in &self.origins {
            let now_size = origin.len();
            total_size += now_size;
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
