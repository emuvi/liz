use crate::liz_forms::{self, Form};

pub trait Parser {
    fn parse(&self, text: &str) -> Vec<Form>;
}

pub struct BlockParser<'a> {
    pub order: Vec<&'a BlockKind<'a>>,
}

pub enum BlockKind<'a> {
    BlockAmid(KindAmid<'a>),
    BlockTick(KindTick<'a>),
    BlockNear(KindChar<'a>),
    BlockEach(KindChar<'a>),
}

pub struct KindChar<'a> {
    pub list: &'a [char],
}

impl<'a> KindChar<'a> {
    pub fn has(&self, check: char) -> bool {
        self.list.iter().any(|ch| *ch == check)
    }
}

pub struct KindAmid<'a> {
    pub opener: KindChar<'a>,
    pub closer: KindChar<'a>,
    pub escape: KindChar<'a>,
}

pub struct KindTick<'a> {
    pub opener: KindChar<'a>,
    pub others: KindChar<'a>,
}

pub static BLOCK_SINGLE_QUOTES: BlockKind<'static> = BlockKind::BlockAmid(KindAmid {
    opener: KindChar { list: &['\''] },
    closer: KindChar { list: &['\''] },
    escape: KindChar { list: &['\\'] },
});

pub static BLOCK_DOUBLE_QUOTES: BlockKind<'static> = BlockKind::BlockAmid(KindAmid {
    opener: KindChar { list: &['"'] },
    closer: KindChar { list: &['"'] },
    escape: KindChar { list: &['\\'] },
});

pub static BLOCK_ANGLE_BRACKET: BlockKind<'static> = BlockKind::BlockAmid(KindAmid {
    opener: KindChar { list: &['<'] },
    closer: KindChar {
        list: &['>', ' ', '\t', '\n', '\r'],
    },
    escape: KindChar { list: &[] },
});

pub static BLOCK_NUMBERS: BlockKind<'static> = BlockKind::BlockTick(KindTick {
    opener: KindChar {
        list: &['-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
    },
    others: KindChar {
        list: &[
            '_', '.', ',', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        ],
    },
});

pub static BLOCK_LINE_SPACE: BlockKind<'static> = BlockKind::BlockNear(KindChar {
    list: liz_forms::LINE_SPACE_CHARS,
});

pub static BLOCK_LINE_BREAK: BlockKind<'static> = BlockKind::BlockNear(KindChar {
    list: liz_forms::LINE_BREAK_CHARS,
});

pub static BLOCK_CODE_BRACKETS: BlockKind<'static> = BlockKind::BlockEach(KindChar {
    list: &['(', ')', '[', ']', '{', '}'],
});

pub static BLOCK_TEXT_BRACKETS: BlockKind<'static> = BlockKind::BlockEach(KindChar {
    list: &['(', ')', '[', ']', '{', '}', '<', '>'],
});

pub static BLOCK_TEXT_QUOTATION: BlockKind<'static> =
    BlockKind::BlockEach(KindChar { list: &['\'', '"'] });

pub static CODE_PARSER: BlockParser<'static> = BlockParser {
    order: vec![
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
    order: vec![
        &BLOCK_NUMBERS,
        &BLOCK_LINE_SPACE,
        &BLOCK_LINE_BREAK,
        &BLOCK_TEXT_BRACKETS,
        &BLOCK_TEXT_QUOTATION,
    ],
};

impl<'a> Parser for BlockParser<'a> {
    fn parse(&self, text: &str) -> Vec<Form> {
        let mut result = Vec::new();
        let mut accrued = String::new();
        let mut inside: Option<&BlockKind> = None;
        let mut previous = '\0';
        for actual in text.chars() {
            if let Some(ref closes_block) = inside {
                let closes_block = *closes_block;
                match closes_block {
                    BlockKind::BlockAmid(amid) => {}
                    BlockKind::BlockTick(tick) => {}
                    BlockKind::BlockNear(near) => {}
                    BlockKind::BlockEach(_) => {}
                }
            } else {
                for opens_block in &self.order {
                    let opens_block = *opens_block;
                    match opens_block {
                        BlockKind::BlockAmid(amid) => {}
                        BlockKind::BlockTick(tick) => {}
                        BlockKind::BlockNear(near) => {}
                        BlockKind::BlockEach(each) => {}
                    }
                }
            }
            accrued.push(actual);
            previous = actual;
        }
        result
    }
}
