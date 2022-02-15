use crate::liz_parse::Parser;

#[derive(Clone)]
pub struct Forms {
    pub list: Vec<Form>,
}

#[derive(Clone)]
pub struct Form {
    pub part: String,
}

impl Forms {
    pub fn parse(text: &str, parser: &impl Parser) -> Forms {
        Forms { list: parser.eval(text) }
    }

    pub fn put(&mut self, part: &str) {
        if !part.is_empty() {
            self.list.push(Form::new(part));
        }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn get(&self, index: usize) -> &str {
        self.list[index].part.as_ref()
    }

    pub fn build(&self) -> String {
        let mut result = String::new();
        for slab in &self.list {
            result.push_str(&slab.part);
        }
        result
    }
}

impl Form {
    pub fn new(part: &str) -> Form {
        Form { part: part.into() }
    }

    pub fn is_whitespace(&self) -> bool {
        !self.part.chars().any(|ch| !ch.is_whitespace())
    }

    pub fn is_linespace(&self) -> bool {
        !self.part.chars().any(|ch| LINE_SPACE_CHARS.iter().any(|lsc| ch != *lsc))
    }

    pub fn is_linebreak(&self) -> bool {
        !self.part.chars().any(|ch| LINE_BREAK_CHARS.iter().any(|lbc| ch != *lbc))
    }
}

pub static LINE_SPACE_CHARS: &[char] = &[' ', '\t'];
pub static LINE_BREAK_CHARS: &[char] = &['\n', '\r'];