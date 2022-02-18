use regex::Regex;
use std::collections::HashMap;

use crate::liz_forms::{self, Form};

pub trait Parser {
    fn parse(&self, text: &str) -> Vec<Form>;
}

pub struct BlockParser<'a> {
    pub blocks: &'a [&'a BlockKind<'a>],
}

pub enum BlockKind<'a> {
    BlockEach(KindChar<'a>),
    BlockNear(KindChar<'a>),
    BlockAmid(KindAmid<'a>),
    BlockTick(KindTick<'a>),
    BlockRexp(&'a str),
}

pub enum KindChar<'a> {
    Control,
    Digit(u32),
    Numeric,
    Alphabetic,
    AlphaNumeric,
    LowerCase,
    UpperCase,
    WhiteSpace,
    InList(KindList<'a>),
}

pub struct KindList<'a> {
    pub list: &'a [char],
}

/// It is inclusive on opener and closer.
/// Wich means it will include the opener and closer steps to the block.
pub struct KindAmid<'a> {
    pub opener: KindChar<'a>,
    pub closer: KindChar<'a>,
    pub escape: KindChar<'a>,
}

/// It is inclusive on opener and exclusive on closer.
/// Wich means it will include the step on open to the block but not on close.
pub struct KindTick<'a> {
    pub opener: &'a [Do<'a>],
    pub closer: &'a [Do<'a>],
}

pub enum Do<'a> {
    Ask(If, Is, KindChar<'a>),
    Tie(Knot),
}

#[derive(PartialEq)]
pub enum If {
    Past,
    Step,
    Next,
}

#[derive(PartialEq)]
pub enum Is {
    Of,
    No,
}

#[derive(PartialEq)]
pub enum Knot {
    And,
    Or,
}

impl<'a> KindChar<'a> {
    pub fn check(&self, over: char) -> bool {
        match &self {
            KindChar::Control => over.is_control(),
            KindChar::Digit(radix) => over.is_digit(*radix),
            KindChar::Numeric => over.is_numeric(),
            KindChar::Alphabetic => over.is_alphabetic(),
            KindChar::AlphaNumeric => over.is_alphanumeric(),
            KindChar::LowerCase => over.is_lowercase(),
            KindChar::UpperCase => over.is_uppercase(),
            KindChar::WhiteSpace => over.is_whitespace(),
            KindChar::InList(list) => list.has(over),
        }
    }
}

impl<'a> KindList<'a> {
    pub fn has(&self, check: char) -> bool {
        self.list.iter().any(|ch| *ch == check)
    }
}

impl<'a> KindTick<'a> {
    pub fn check_opens(&self, past: char, step: char, next: char) -> bool {
        KindTick::check(self.opener, past, step, next)
    }

    pub fn check_closes(&self, past: char, step: char, next: char) -> bool {
        KindTick::check(self.closer, past, step, next)
    }

    fn check(fusion: &'a [Do<'a>], past: char, step: char, next: char) -> bool {
        let mut stack: Vec<bool> = Vec::with_capacity(fusion.len());
        for act in (*fusion).iter() {
            match act {
                Do::Ask(var, like, tester) => {
                    let over = match var {
                        If::Past => past,
                        If::Step => step,
                        If::Next => next,
                    };
                    let mut partial = tester.check(over);
                    if *like == Is::No {
                        partial = !partial;
                    }
                    stack.push(partial);
                }
                Do::Tie(joint) => {
                    let last = if let Some(last) = stack.pop() {
                        last
                    } else {
                        false
                    };
                    let penult = if let Some(last) = stack.pop() {
                        last
                    } else {
                        false
                    };
                    match joint {
                        Knot::And => {stack.push(last && penult);},
                        Knot::Or => {stack.push(last || penult);},
                    }
                }
            }
        }
        match stack.pop() {
            Some(result) => result,
            None => false,
        }
    }
}

pub static BLOCK_SINGLE_QUOTES: BlockKind<'static> = BlockKind::BlockAmid(KindAmid {
    opener: KindChar::InList(KindList { list: &['\''] }),
    closer: KindChar::InList(KindList { list: &['\''] }),
    escape: KindChar::InList(KindList { list: &['\\'] }),
});

pub static BLOCK_DOUBLE_QUOTES: BlockKind<'static> = BlockKind::BlockAmid(KindAmid {
    opener: KindChar::InList(KindList { list: &['"'] }),
    closer: KindChar::InList(KindList { list: &['"'] }),
    escape: KindChar::InList(KindList { list: &['\\'] }),
});

pub static BLOCK_ANGLE_BRACKET: BlockKind<'static> = BlockKind::BlockAmid(KindAmid {
    opener: KindChar::InList(KindList { list: &['<'] }),
    closer: KindChar::InList(KindList {
        list: &['>', ' ', '\t', '\n', '\r'],
    }),
    escape: KindChar::InList(KindList { list: &[] }),
});

pub static BLOCK_REGULAR: BlockKind<'static> = BlockKind::BlockTick(KindTick {
    opener: &[Do::Ask(If::Step, Is::Of, KindChar::Alphabetic)],
    closer: &[Do::Ask(If::Step, Is::No, KindChar::AlphaNumeric)],
});

pub static BLOCK_NUMBERS: BlockKind<'static> = BlockKind::BlockTick(KindTick {
    opener: &[Do::Ask(If::Step, Is::Of, KindChar::Digit(10))],
    closer: &[Do::Ask(If::Step, Is::No, KindChar::Digit(10))],
});

pub static BLOCK_LINE_SPACE: BlockKind<'static> =
    BlockKind::BlockNear(KindChar::InList(KindList {
        list: liz_forms::LINE_SPACE_CHARS,
    }));

pub static BLOCK_LINE_BREAK: BlockKind<'static> =
    BlockKind::BlockNear(KindChar::InList(KindList {
        list: liz_forms::LINE_BREAK_CHARS,
    }));

pub static BLOCK_CODE_BRACKETS: BlockKind<'static> =
    BlockKind::BlockEach(KindChar::InList(KindList {
        list: &['(', ')', '[', ']', '{', '}'],
    }));

pub static BLOCK_TEXT_BRACKETS: BlockKind<'static> =
    BlockKind::BlockEach(KindChar::InList(KindList {
        list: &['(', ')', '[', ']', '{', '}', '<', '>'],
    }));

pub static BLOCK_TEXT_QUOTATION: BlockKind<'static> =
    BlockKind::BlockEach(KindChar::InList(KindList { list: &['\'', '"'] }));

pub static CODE_PARSER: BlockParser<'static> = BlockParser {
    blocks: &[
        &BLOCK_SINGLE_QUOTES,
        &BLOCK_DOUBLE_QUOTES,
        &BLOCK_ANGLE_BRACKET,
        &BLOCK_REGULAR,
        &BLOCK_NUMBERS,
        &BLOCK_LINE_SPACE,
        &BLOCK_LINE_BREAK,
        &BLOCK_CODE_BRACKETS,
    ],
};

pub static TEXT_PARSER: BlockParser<'static> = BlockParser {
    blocks: &[
        &BLOCK_REGULAR,
        &BLOCK_NUMBERS,
        &BLOCK_LINE_SPACE,
        &BLOCK_LINE_BREAK,
        &BLOCK_TEXT_BRACKETS,
        &BLOCK_TEXT_QUOTATION,
    ],
};

struct BlockParserHelper {
    result: Vec<Form>,
    accrued: String,
}

impl BlockParserHelper {
    fn new() -> Self {
        Self {
            result: Vec::new(),
            accrued: String::new(),
        }
    }

    fn accrue_char(&mut self, brick: char) {
        self.accrued.push(brick);
    }

    fn accrue_undo(&mut self) {
        self.accrued.pop();
    }

    fn commit_accrued(&mut self) {
        if !self.accrued.is_empty() {
            self.result.push(Form::from(self.accrued.clone()));
            self.accrued.clear();
        }
    }

    fn got_form(&mut self, from: String) {
        self.result.push(Form::from(from));
    }
}

impl<'a> Parser for BlockParser<'a> {
    fn parse(&self, text: &str) -> Vec<Form> {
        let mut helper = BlockParserHelper::new();
        let mut crexps: HashMap<&str, Regex> = HashMap::new();
        for block in self.blocks {
            let block = *block;
            match block {
                BlockKind::BlockRexp(regex) => {
                    let crexp = Regex::new(regex).unwrap();
                    crexps.insert(regex, crexp);
                }
                _ => {}
            }
        }
        let mut inside_block: Option<&BlockKind> = None;
        let chars: Vec<char> = text.chars().collect();
        let mut index = 0;
        while index < chars.len() {
            let mut accrue_char = true;
            let mut check_block_opens = true;
            let step = *chars.get(index).unwrap();
            let past = if index > 0 {
                *chars.get(index - 1).unwrap()
            } else {
                '\0'
            };
            let next = if index < chars.len() - 1 {
                *chars.get(index + 1).unwrap()
            } else {
                '\0'
            };
            if let Some(ref closes_block) = inside_block {
                let closes_block = *closes_block;
                match closes_block {
                    BlockKind::BlockEach(_) => {}
                    BlockKind::BlockNear(near) => {
                        if !near.check(step) {
                            helper.commit_accrued();
                            inside_block = None;
                        }
                    }
                    BlockKind::BlockAmid(amid) => {
                        if amid.closer.check(step) && !amid.escape.check(past) {
                            helper.accrue_char(step);
                            helper.commit_accrued();
                            accrue_char = false;
                            check_block_opens = false;
                            inside_block = None;
                        }
                    }
                    BlockKind::BlockTick(tick) => {
                        if tick.check_closes(past, step, next) {
                            helper.commit_accrued();
                            inside_block = None;
                        }
                    }
                    BlockKind::BlockRexp(regex) => {
                        let crexp = crexps.get(regex).unwrap();
                        helper.accrue_char(step);
                        let commit = !crexp.is_match(&helper.accrued);
                        helper.accrue_undo();
                        if commit {
                            helper.commit_accrued();
                            inside_block = None;
                        }
                    }
                }
            }
            if check_block_opens && inside_block.is_none() {
                for opens_block in self.blocks {
                    let opens_block = *opens_block;
                    match opens_block {
                        BlockKind::BlockEach(each) => {
                            if each.check(step) {
                                helper.commit_accrued();
                                helper.got_form(String::from(step));
                                accrue_char = false;
                                break;
                            }
                        }
                        BlockKind::BlockNear(near) => {
                            if near.check(step) {
                                helper.commit_accrued();
                                inside_block = Some(opens_block);
                                break;
                            }
                        }
                        BlockKind::BlockAmid(amid) => {
                            if amid.opener.check(step) {
                                helper.commit_accrued();
                                inside_block = Some(opens_block);
                                break;
                            }
                        }
                        BlockKind::BlockTick(tick) => {
                            if tick.check_opens(past, step, next) {
                                helper.commit_accrued();
                                inside_block = Some(opens_block);
                                break;
                            }
                        }
                        BlockKind::BlockRexp(regex) => {
                            let crexp = crexps.get(regex).unwrap();
                            if crexp.is_match(&helper.accrued) {
                                inside_block = Some(opens_block);
                                break;
                            }
                        }
                    }
                }
            }
            if accrue_char {
                helper.accrue_char(step);
            }
            index += 1;
        }
        helper.commit_accrued();
        helper.result
    }
}

#[test]
fn test_code_parser() {
    let test01 = "token1 123.+token2";
    let results = CODE_PARSER.parse(test01);
    println!("{:?}", results);
}
