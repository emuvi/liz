#[derive(Debug, Clone, PartialEq)]
pub struct Forms {
    pub list: Vec<Form>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Form {
    pub part: String,
}

impl Forms {
    pub fn new(list: Vec<Form>) -> Forms {
        Forms { list }
    }

    pub fn from(slice: &[impl AsRef<str>]) -> Forms {
        let mut list: Vec<Form> = Vec::with_capacity(slice.len());
        for item in slice {
            list.push(Form::new(item.as_ref()));
        }
        Forms { list }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn get(&self, index: usize) -> &Form {
        &self.list[index]
    }

    pub fn put(&mut self, part: &str) {
        if !part.is_empty() {
            self.list.push(Form::new(part));
        }
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

    pub fn from(part: String) -> Form {
        Form { part }
    }

    pub fn is_whitespace(&self) -> bool {
        !self.part.chars().any(|ch| !ch.is_whitespace())
    }

    pub fn is_linespace(&self) -> bool {
        !self
            .part
            .chars()
            .any(|ch| LINE_SPACE_CHARS.iter().any(|item| ch != *item))
    }

    pub fn is_linebreak(&self) -> bool {
        !self
            .part
            .chars()
            .any(|ch| LINE_BREAK_CHARS.iter().any(|item| ch != *item))
    }
}

pub static LINE_SPACE_CHARS: &[char] = &[' ', '\t'];

pub static LINE_BREAK_CHARS: &[char] = &['\n', '\r'];

pub static CODE_BRACKETS_CHARS: &[char] = &['(', ')', '[', ']', '{', '}'];

pub static TEXT_BRACKETS_CHARS: &[char] = &['(', ')', '[', ']', '{', '}', '<', '>'];

pub static TEXT_QUOTATION_CHARS: &[char] = &['\'', '"'];
