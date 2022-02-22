use crate::LizError;
use crate::liz_texts;

#[derive(Debug, Clone, PartialEq)]
pub struct Forms {
    pub desk: Vec<Form>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Form {
    pub term: String,
}

impl Forms {
    pub fn edit() -> Forms {
        Forms { desk: Vec::new() }
    }

    pub fn new(desk: Vec<Form>) -> Forms {
        Forms { desk }
    }

    pub fn from(slice: &[impl AsRef<str>]) -> Forms {
        let mut desk: Vec<Form> = Vec::with_capacity(slice.len());
        for item in slice {
            desk.push(Form::new(item.as_ref()));
        }
        Forms { desk }
    }

    pub fn len(&self) -> usize {
        self.desk.len()
    }

    pub fn get(&self, index: usize) -> &Form {
        &self.desk[index]
    }

    pub fn set(&mut self, index: usize, form: Form) {
        self.desk[index] = form;
    }

    pub fn add(&mut self, index: usize, form: Form) {
        self.desk.insert(index, form)
    }

    pub fn put(&mut self, form: Form) {
        self.desk.push(form)
    }

    pub fn del(&mut self, index: usize) -> Form {
        self.desk.remove(index)
    }

    pub fn pop(&mut self) -> Option<Form> {
        self.desk.pop()
    }

    pub fn change_all(&mut self, of: &str, to: &str) {
        for form in &mut self.desk {
            if form.term == of {
                form.term = to.into();
            }
        }
    }

    pub fn print_all(&self) {
        print!("[");
        let mut first = true;
        for form in &self.desk {
            if first {
                first = false;
            } else {
                print!(",")        
            }
            form.print();
        }
        println!("]");
    }

    pub fn build(&self) -> String {
        let mut result = String::new();
        for form in &self.desk {
            result.push_str(&form.term);
        }
        result
    }

    pub fn write(&self, path: &str) -> Result<(), LizError> {
        let contents = self.build();
        liz_texts::write(path, &contents)
    }
}

impl Form {
    pub fn new(term: &str) -> Form {
        Form { term: term.into() }
    }

    pub fn from(term: String) -> Form {
        Form { term }
    }

    pub fn print(&self) {
        print!("'{}'", self.term);
    }

    pub fn is_whitespace(&self) -> bool {
        !self.term.chars().any(|ch| !ch.is_whitespace())
    }

    pub fn is_linespace(&self) -> bool {
        !self
            .term
            .chars()
            .any(|ch| LINE_SPACE_CHARS.iter().any(|item| ch != *item))
    }

    pub fn is_linebreak(&self) -> bool {
        !self
            .term
            .chars()
            .any(|ch| LINE_BREAK_CHARS.iter().any(|item| ch != *item))
    }

    pub fn is_code_brackets(&self) -> bool {
        !self
            .term
            .chars()
            .any(|ch| CODE_BRACKETS_CHARS.iter().any(|item| ch != *item))
    }

    pub fn is_text_brackets(&self) -> bool {
        !self
            .term
            .chars()
            .any(|ch| TEXT_BRACKETS_CHARS.iter().any(|item| ch != *item))
    }

    pub fn is_text_quotation(&self) -> bool {
        !self
            .term
            .chars()
            .any(|ch| TEXT_QUOTATION_CHARS.iter().any(|item| ch != *item))
    }
}

pub static LINE_SPACE_CHARS: &[char] = &[' ', '\t'];

pub static LINE_BREAK_CHARS: &[char] = &['\n', '\r'];

pub static CODE_BRACKETS_CHARS: &[char] = &['(', ')', '[', ']', '{', '}'];

pub static TEXT_BRACKETS_CHARS: &[char] = &['(', ')', '[', ']', '{', '}', '<', '>'];

pub static TEXT_QUOTATION_CHARS: &[char] = &['\'', '"'];
