use crate::liz_paths;

#[derive(Clone)]
pub struct Tokenizer {}

#[derive(Clone)]
pub struct Slabs {
    list: Vec<Slab>,
    kind: Kind,
}

#[derive(Clone)]
struct Slab {
    part: String,
}

#[derive(Clone)]
enum Kind {
    C,
    Lisp,
    Text,
}

impl Slabs {
    pub fn parse(text: &str, name: &str) -> Slabs {
        let mut result = Slabs::new(Kind::from(name));
        let mut part = String::new();
        let mut last_was_space = false;
        for ch in text.chars() {
            if ch == ' ' {
                if !last_was_space {
                    last_was_space = true;
                    result.put(&part);
                    part.clear();
                }
            } else {
                if last_was_space {
                    last_was_space = false;
                    result.put(&part);
                    part.clear();
                }
            }
            part.push(ch);
        }
        result.put(&part);
        result
    }

    fn new(kind: Kind) -> Slabs {
        Slabs {
            list: Vec::new(),
            kind,
        }
    }

    pub fn put(&mut self, part: &str) {
        if !part.is_empty() {
            self.list.push(Slab::new(part.into()));
        }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn get(&self, index: usize) -> String {
        self.list[index].part.clone()
    }

    pub fn build(&self) -> String {
        let mut result = String::new();
        for slab in &self.list {
            result.push_str(&slab.part);
        }
        result
    }
}

impl Slab {
    fn new(part: String) -> Slab {
        Slab { part }
    }

    fn is_space(&self) -> bool {
        self.part.trim().is_empty()
    }
}

impl Kind {
    fn from(name: &str) -> Kind {
        if liz_paths::path_ext_is_on(name, &[".txt", ".md"]) {
            Kind::Text
        } else if liz_paths::path_ext_is_on(name, &[".el"]) {
            Kind::Lisp
        } else {
            Kind::C
        }
    }
}
