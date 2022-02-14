use crate::liz_forms::Form;

pub trait Parser {
    fn eval(&self, text: &str) -> Vec<Form>;
}

pub struct DefaultParser {
    pub group_single_quotes: bool,
    pub group_double_quotes: bool,
    pub group_non_alfanumeric: bool,
}

pub static DEFAULT_PARSER: DefaultParser = DefaultParser {
    group_single_quotes: true,
    group_double_quotes: true,
    group_non_alfanumeric: true,
};

impl Parser for DefaultParser {
    fn eval(&self, text: &str) -> Vec<Form> {
        Vec::new()
    }
}
