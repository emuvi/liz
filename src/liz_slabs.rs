#[derive(Clone)]
pub struct Slabs {
    list: Vec<Slab>,
}

#[derive(Clone)]
struct Slab {
    part: String,
}

impl Slabs {
    pub fn new() -> Slabs {
        Slabs { list: Vec::new() }
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
}

pub fn parse(text: &str, name: &str) -> Slabs {
    let mut result = Slabs::new();
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
