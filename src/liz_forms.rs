use crate::liz_debug::dbg_bleb;
use crate::liz_debug::{dbg_call, dbg_reav, dbg_seal, dbg_step, dbg_tell};
use crate::liz_texts;
use crate::LizError;

#[derive(Debug, Clone, PartialEq)]
pub struct Forms {
    pub desk: Vec<Form>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Form {
    pub term: String,
}

impl Forms {
    pub fn new() -> Forms {
        dbg_reav!(Forms { desk: Vec::new() });
    }

    pub fn with(desk: Vec<Form>) -> Forms {
        dbg_reav!(Forms { desk });
    }

    pub fn take(terms: Vec<String>) -> Forms {
        dbg_call!(terms);
        let mut desk: Vec<Form> = Vec::with_capacity(terms.len());
        for term in terms {
            desk.push(dbg_step!(Form::with(term)));
        }
        dbg_reav!(Forms { desk });
    }

    pub fn from(terms: &[impl AsRef<str> + std::fmt::Debug]) -> Forms {
        dbg_call!(terms);
        let mut desk: Vec<Form> = Vec::with_capacity(terms.len());
        for term in terms {
            desk.push(dbg_step!(Form::from(term.as_ref())));
        }
        dbg_reav!(Forms { desk });
    }

    pub fn len(&self) -> usize {
        dbg_call!();
        dbg_reav!(self.desk.len());
    }

    pub fn get(&self, index: usize) -> &Form {
        dbg_call!(index);
        dbg_reav!(&self.desk[index]);
    }

    pub fn set(&mut self, index: usize, form: Form) {
        dbg_call!(index, form);
        self.desk[index] = form;
    }

    pub fn add(&mut self, index: usize, form: Form) {
        dbg_call!(index, form);
        self.desk.insert(index, form)
    }

    pub fn put(&mut self, form: Form) {
        dbg_call!(form);
        self.desk.push(form)
    }

    pub fn del(&mut self, index: usize) -> Form {
        dbg_call!(index);
        dbg_reav!(self.desk.remove(index));
    }

    pub fn pop(&mut self) -> Option<Form> {
        dbg_call!();
        dbg_reav!(self.desk.pop());
    }

    pub fn find_all(&self, term: &str) -> Vec<usize> {
        dbg_call!(term);
        dbg_reav!(self.find_all_pred(|form| { form.is_equals(term) }));
    }

    pub fn find_all_like(&self, term: &str) -> Vec<usize> {
        dbg_call!(term);
        dbg_reav!(self.find_all_pred(|form| { form.is_likely(term) }));
    }

    pub fn find_all_pred<F: Fn(&Form) -> bool>(&self, pred: F) -> Vec<usize> {
        dbg_call!();
        let mut result = Vec::new();
        let mut index = 0;
        for form in &self.desk {
            if pred(form) {
                dbg_tell!(form, index);
                result.push(index);
            }
            index += 1;
        }
        dbg_reav!(result);
    }

    pub fn next_not_space(&self, of: usize) -> Option<usize> {
        dbg_call!();
        let mut index = of + 1;
        while index < self.desk.len() {
            if !self.desk[index].is_whitespace() {
                dbg_reav!(Some(index));
            }
            index += 1;
        }
        dbg_reav!(None);
    }

    pub fn change_all(&mut self, of: &str, to: &str) {
        dbg_call!(of, to);
        for form in &mut self.desk {
            if form.term == of {
                form.term = to.into();
            }
        }
    }

    pub fn print(&self, index: usize) {
        dbg_call!();
        self.desk[index].print();
    }

    pub fn println(&self, index: usize) {
        dbg_call!();
        self.desk[index].println();
    }

    pub fn print_all(&self) {
        dbg_call!();
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
        dbg_call!();
        let mut result = String::new();
        for form in &self.desk {
            result.push_str(&form.term);
        }
        dbg_reav!(result);
    }

    pub fn write(&self, path: &str) -> Result<(), LizError> {
        dbg_call!(path);
        let contents = self.build();
        dbg_seal!(contents);
        liz_texts::write(path, contents).map_err(|err| dbg_bleb!(err))
    }
}

impl Form {
    pub fn new() -> Form {
        dbg_call!();
        dbg_reav!(Form {
            term: String::default()
        });
    }

    pub fn with(term: String) -> Form {
        dbg_call!(term);
        dbg_reav!(Form { term });
    }

    pub fn from(term: &str) -> Form {
        dbg_call!(term);
        dbg_reav!(Form { term: term.into() });
    }

    pub fn is_equals(&self, term: &str) -> bool {
        dbg_call!(term);
        dbg_reav!(self.term == term);
    }

    pub fn is_likely(&self, term: &str) -> bool {
        dbg_call!(term);
        dbg_reav!(liz_texts::is_likely(&self.term, term));
    }

    pub fn print(&self) {
        dbg_call!();
        print!("'{}'", self.term);
    }

    pub fn println(&self) {
        dbg_call!();
        println!("'{}'", self.term);
    }

    pub fn is_whitespace(&self) -> bool {
        dbg_call!();
        dbg_reav!(!self.term.chars().any(|ch| !ch.is_whitespace()));
    }

    pub fn is_linespace(&self) -> bool {
        dbg_call!();
        dbg_reav!(!self
            .term
            .chars()
            .any(|ch| LINE_SPACE_CHARS.iter().any(|item| ch != *item)));
    }

    pub fn is_linebreak(&self) -> bool {
        dbg_call!();
        dbg_reav!(!self
            .term
            .chars()
            .any(|ch| LINE_BREAK_CHARS.iter().any(|item| ch != *item)));
    }

    pub fn is_code_brackets(&self) -> bool {
        dbg_call!();
        dbg_reav!(!self
            .term
            .chars()
            .any(|ch| CODE_BRACKETS_CHARS.iter().any(|item| ch != *item)));
    }

    pub fn is_text_brackets(&self) -> bool {
        dbg_call!();
        dbg_reav!(!self
            .term
            .chars()
            .any(|ch| TEXT_BRACKETS_CHARS.iter().any(|item| ch != *item)));
    }

    pub fn is_text_quotation(&self) -> bool {
        dbg_call!();
        dbg_reav!(!self
            .term
            .chars()
            .any(|ch| TEXT_QUOTATION_CHARS.iter().any(|item| ch != *item)));
    }
}

pub static LINE_SPACE_CHARS: &[char] = &[' ', '\t'];

pub static LINE_BREAK_CHARS: &[char] = &['\n', '\r'];

pub static CODE_BRACKETS_CHARS: &[char] = &['(', ')', '[', ']', '{', '}'];

pub static TEXT_BRACKETS_CHARS: &[char] = &['(', ')', '[', ']', '{', '}', '<', '>'];

pub static TEXT_QUOTATION_CHARS: &[char] = &['\'', '"'];
