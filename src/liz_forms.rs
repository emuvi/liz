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

    pub fn set(&mut self, index: usize, form: Form) {
        self.list[index] = form;
    }

    pub fn add(&mut self, index: usize, form: Form) {
        self.list.insert(index, form)
    }

    pub fn put(&mut self, form: Form) {
        self.list.push(form)
    }

    pub fn del(&mut self, index: usize) -> Form {
        self.list.remove(index)
    }

    pub fn pop(&mut self) -> Option<Form> {
        self.list.pop()
    }

    pub fn change_all(&mut self, of: &str, to: &str) {
        for form in &mut self.list {
            if form.part == of {
                form.part = to.into();
            }
        }
    }

    pub fn build(&self) -> String {
        let mut result = String::new();
        for form in &self.list {
            result.push_str(&form.part);
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

    pub fn is_code_brackets(&self) -> bool {
        !self
            .part
            .chars()
            .any(|ch| CODE_BRACKETS_CHARS.iter().any(|item| ch != *item))
    }

    pub fn is_text_brackets(&self) -> bool {
        !self
            .part
            .chars()
            .any(|ch| TEXT_BRACKETS_CHARS.iter().any(|item| ch != *item))
    }

    pub fn is_text_quotation(&self) -> bool {
        !self
            .part
            .chars()
            .any(|ch| TEXT_QUOTATION_CHARS.iter().any(|item| ch != *item))
    }
}

pub static LINE_SPACE_CHARS: &[char] = &[' ', '\t'];

pub static LINE_BREAK_CHARS: &[char] = &['\n', '\r'];

pub static CODE_BRACKETS_CHARS: &[char] = &['(', ')', '[', ']', '{', '}'];

pub static TEXT_BRACKETS_CHARS: &[char] = &['(', ')', '[', ']', '{', '}', '<', '>'];

pub static TEXT_QUOTATION_CHARS: &[char] = &['\'', '"'];
