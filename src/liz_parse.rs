use crate::liz_forms::Form;

pub trait Parser {
    fn eval(&self, text: &str) -> Vec<Form>;
}

pub struct DefaultParser {
    code_like: bool
}

pub static CODE_PARSER: DefaultParser = DefaultParser {
    code_like: true
};

pub static TEXT_PARSER: DefaultParser = DefaultParser {
    code_like: false
};

impl Parser for DefaultParser {
    fn eval(&self, text: &str) -> Vec<Form> {
        let mut result = Vec::new();
        let mut part = String::new();
        for ch in text.chars() {
            if self.code_like {
                part.push(ch);
            }
        }
        if !part.is_empty() {
            result.push(Form::new(&part));
        }
        result
    }
}
