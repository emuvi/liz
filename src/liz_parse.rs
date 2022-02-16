use regex::Regex;
use std::collections::HashMap;

use crate::liz_forms::{self, Form};

pub trait Parser {
    fn parse(&self, text: &str) -> Vec<Form>;
}

pub struct BlockParser<'a> {
    pub order: &'a [&'a BlockKind<'a>],
}

pub enum BlockKind<'a> {
    BlockAmid(KindAmid<'a>),
    BlockNear(KindChar<'a>),
    BlockEach(KindChar<'a>),
    BlockDigit(KindDigit<'a>),
    BlockRegex(&'a str),
}

pub enum KindChar<'a> {
    IsControl,
    IsDigit(u32),
    IsNumeric,
    IsAlphabetic,
    IsAlphaNumeric,
    IsLowerCase,
    IsUpperCase,
    IsWhiteSpace,
    IsInList(KindList<'a>),
}

impl<'a> KindChar<'a> {
    pub fn check(&self, over: char) -> bool {
        match &self {
            KindChar::IsControl => over.is_control(),
            KindChar::IsDigit(radix) => over.is_digit(*radix),
            KindChar::IsNumeric => over.is_numeric(),
            KindChar::IsAlphabetic => over.is_alphabetic(),
            KindChar::IsAlphaNumeric => over.is_alphanumeric(),
            KindChar::IsLowerCase => over.is_lowercase(),
            KindChar::IsUpperCase => over.is_uppercase(),
            KindChar::IsWhiteSpace => over.is_whitespace(),
            KindChar::IsInList(list) => list.has(over)
        }
    }
}

pub struct KindList<'a> {
    pub list: &'a [char],
}

impl<'a> KindList<'a> {
    pub fn has(&self, check: char) -> bool {
        self.list.iter().any(|ch| *ch == check)
    }
}

pub struct KindAmid<'a> {
    pub opener: KindChar<'a>,
    pub closer: KindChar<'a>,
    pub escape: KindChar<'a>,
}

pub struct KindDigit<'a> {
    pub can_start_on: KindChar<'a>,
    pub main_must_be: KindChar<'a>,
    pub can_have_too: KindChar<'a>,
}

pub struct KindTick<'a> {
    pub prior: Option<&'a [KindTest<'a>]>,
    pub actual: Option<&'a [KindTest<'a>]>,
    pub next: Option<&'a [KindTest<'a>]>,
}

pub struct KindTest<'a> {
    pub invert: bool,
    pub tester: KindChar<'a>,
    pub joints: Option<KindJoint>
}

pub enum KindJoint {
    And, Or
}

impl<'a> KindTick<'a> {
    pub fn begins(prior: char, actual: char) -> bool {false}
    pub fn ends(actual: char, next: char) -> bool {false}
}

pub static BLOCK_SINGLE_QUOTES: BlockKind<'static> = BlockKind::BlockAmid(KindAmid {
    opener: KindChar::IsInList(KindList{ list: &['\''] }),
    closer: KindChar::IsInList(KindList{ list: &['\''] }),
    escape: KindChar::IsInList(KindList{ list: &['\\'] }),
});

pub static BLOCK_DOUBLE_QUOTES: BlockKind<'static> = BlockKind::BlockAmid(KindAmid {
    opener: KindChar::IsInList(KindList{ list: &['"'] }),
    closer: KindChar::IsInList(KindList{ list: &['"'] }),
    escape: KindChar::IsInList(KindList{ list: &['\\'] }),
});

pub static BLOCK_ANGLE_BRACKET: BlockKind<'static> = BlockKind::BlockAmid(KindAmid {
    opener: KindChar::IsInList(KindList{ list: &['<'] }),
    closer: KindChar::IsInList(KindList{
        list: &['>', ' ', '\t', '\n', '\r'],
    }),
    escape: KindChar::IsInList(KindList{ list: &[] }),
});

pub static BLOCK_NUMBERS: BlockKind<'static> = BlockKind::BlockDigit(KindDigit {
    can_start_on: KindChar::IsInList(KindList{ list: &['-'] }),
    main_must_be: KindChar::IsInList(KindList{
        list: &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
    }),
    can_have_too: KindChar::IsInList(KindList{
        list: &['.', ',', '_'],
    }),
});

pub static BLOCK_LINE_SPACE: BlockKind<'static> = BlockKind::BlockNear(KindChar::IsInList(KindList{
    list: liz_forms::LINE_SPACE_CHARS,
}));

pub static BLOCK_LINE_BREAK: BlockKind<'static> = BlockKind::BlockNear(KindChar::IsInList(KindList{
    list: liz_forms::LINE_BREAK_CHARS,
}));

pub static BLOCK_CODE_BRACKETS: BlockKind<'static> = BlockKind::BlockEach(KindChar::IsInList(KindList{
    list: &['(', ')', '[', ']', '{', '}'],
}));

pub static BLOCK_TEXT_BRACKETS: BlockKind<'static> = BlockKind::BlockEach(KindChar::IsInList(KindList{
    list: &['(', ')', '[', ']', '{', '}', '<', '>'],
}));

pub static BLOCK_TEXT_QUOTATION: BlockKind<'static> =
    BlockKind::BlockEach(KindChar::IsInList(KindList{ list: &['\'', '"'] }));

pub static CODE_PARSER: BlockParser<'static> = BlockParser {
    order: &[
        &BLOCK_SINGLE_QUOTES,
        &BLOCK_DOUBLE_QUOTES,
        &BLOCK_ANGLE_BRACKET,
        &BLOCK_NUMBERS,
        &BLOCK_LINE_SPACE,
        &BLOCK_LINE_BREAK,
        &BLOCK_CODE_BRACKETS,
    ],
};

pub static TEXT_PARSER: BlockParser<'static> = BlockParser {
    order: &[
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

    fn accrue(&mut self, brick: char) {
        self.accrued.push(brick);
    }

    fn accrue_undo(&mut self) {
        self.accrued.pop();
    }

    fn commit(&mut self) {
        if !self.accrued.is_empty() {
            self.result.push(Form::from(self.accrued.clone()));
            self.accrued.clear();
        }
    }

    fn got(&mut self, from: String) {
        self.result.push(Form::from(from));
    }
}

impl<'a> Parser for BlockParser<'a> {
    fn parse(&self, text: &str) -> Vec<Form> {
        let mut helper = BlockParserHelper::new();
        let mut crexps: HashMap<&str, Regex> = HashMap::new();
        for block in self.order {
            let block = *block;
            match block {
                BlockKind::BlockRegex(regex) => {
                    let crexp = Regex::new(regex).unwrap();
                    crexps.insert(regex, crexp);
                }
                _ => {}
            }
        }
        let mut block_now: Option<&BlockKind> = None;
        let chars: Vec<char> = text.chars().collect();
        let mut index = 0;
        while index < chars.len() {
            let mut accrue = true;
            let mut check_block = true;
            let actual = *chars.get(index).unwrap();
            let prior = if index > 0 {
                *chars.get(index - 1).unwrap()
            } else {
                '\0'
            };
            let next = if index < chars.len() - 1 {
                *chars.get(index + 1).unwrap()
            } else {
                '\0'
            };
            if let Some(ref closes_block) = block_now {
                let closes_block = *closes_block;
                match closes_block {
                    BlockKind::BlockAmid(amid) => {
                        if amid.closer.check(actual) && !amid.escape.check(prior) {
                            helper.accrue(actual);
                            helper.commit();
                            accrue = false;
                            check_block = false;
                            block_now = None;
                        }
                    }
                    BlockKind::BlockNear(near) => {
                        if !near.check(actual) {
                            helper.commit();
                            block_now = None;
                        }
                    }
                    BlockKind::BlockEach(_) => {}
                    BlockKind::BlockDigit(digit) => {
                        if !digit.main_must_be.check(actual) && !digit.can_have_too.check(actual) {
                            helper.commit();
                            block_now = None;
                        }
                    }
                    BlockKind::BlockRegex(regex) => {
                        let crexp = crexps.get(regex).unwrap();
                        helper.accrue(actual);
                        let commit = !crexp.is_match(&helper.accrued);
                        helper.accrue_undo();
                        if commit {
                            helper.commit();
                            block_now = None;
                        }
                    }
                }
            }
            if check_block && block_now.is_none() {
                for opens_block in self.order {
                    let opens_block = *opens_block;
                    match opens_block {
                        BlockKind::BlockAmid(amid) => {
                            if amid.opener.check(actual) {
                                helper.commit();
                                block_now = Some(opens_block);
                            }
                        }
                        BlockKind::BlockNear(near) => {
                            if near.check(actual) {
                                helper.commit();
                                block_now = Some(opens_block);
                            }
                        }
                        BlockKind::BlockEach(each) => {
                            if each.check(actual) {
                                helper.commit();
                                helper.got(String::from(actual));
                                accrue = false;
                            }
                        }
                        BlockKind::BlockDigit(digit) => {
                            if digit.main_must_be.check(actual)
                                || (digit.can_start_on.check(actual) && digit.main_must_be.check(next))
                            {
                                helper.commit();
                                block_now = Some(opens_block);
                            }
                        }
                        BlockKind::BlockRegex(regex) => {
                            let crexp = crexps.get(regex).unwrap();
                            if crexp.is_match(&helper.accrued) {
                                block_now = Some(opens_block);
                            }
                        }
                    }
                }
            }
            if accrue {
                helper.accrue(actual);
            }
            index += 1;
        }
        helper.commit();
        helper.result
    }
}

#[test]
fn test_code_parser() {
    let test01 = "token1  token2";
    let results = CODE_PARSER.parse(test01);
    println!("{:?}", results);
}
