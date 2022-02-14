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

    pub fn is_space(&self) -> bool {
        self.part.trim().is_empty()
    }
}
