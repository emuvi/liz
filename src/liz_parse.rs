use crate::liz_forms;

pub fn parse_group_whitespace(forms: &mut Vec<String>) -> usize {
    parse_group_whitespace_on(forms, 0, liz_forms::forms_len(forms))
}

pub fn parse_group_whitespace_on(forms: &mut Vec<String>, from: usize, till: usize) -> usize {
    parse_group_near_ask_on(forms, from, till, |ch| ch.is_whitespace())
}

pub fn parse_split_whitespace(forms: &mut Vec<String>) -> usize {
    parse_split_whitespace_on(forms, 0, liz_forms::forms_len(forms))
}

pub fn parse_split_whitespace_on(forms: &mut Vec<String>, from: usize, till: usize) -> usize {
    parse_split_near_ask_on(forms, from, till, |ch| ch.is_whitespace())
}

pub fn parse_group_near_ask_on<F: Fn(char) -> bool>(
    forms: &mut Vec<String>,
    from: usize,
    till: usize,
    ask: F,
) -> usize {
    let mut range = parse_del_range(forms, from, till);
    let mut results = Vec::new();
    loop {
        if range.is_empty() {
            break;
        }
        let side_a = range.remove(0);
        if range.is_empty() {
            results.push(side_a);
        } else {
            let side_b = range.remove(0);
            let side_a_first = side_a.chars().next();
            let side_b_last = side_a.chars().last();
            let mut should_group = false;
            if let Some(side_a_first) = side_a_first {
                if let Some(side_b_last) = side_b_last {
                    if ask(side_a_first) != ask(side_b_last) {
                        should_group = true;
                    }
                }
            }
            if should_group {
                results.push(side_a + &side_b);
            } else {
                results.push(side_a);
                results.push(side_b);
            }
        }
    }
    let result = results.len();
    parse_put_range(forms, from, results);
    result
}

pub fn parse_split_near_ask_on<F: Fn(char) -> bool>(
    forms: &mut Vec<String>,
    from: usize,
    till: usize,
    ask: F,
) -> usize {
    let range = parse_del_range(forms, from, till);
    let mut helps = ParserHelper::new();
    let mut state = false;
    for form in range {
        for ch in form.chars() {
            if ask(ch) != state {
                helps.commit_accrued();
                state = !state;
            }
            helps.accrue_char(ch);
        }
    }
    helps.commit_accrued();
    let range = helps.results;
    let result = range.len();
    parse_put_range(forms, from, range);
    result
}

pub fn parse_del_range(forms: &mut Vec<String>, from: usize, till: usize) -> Vec<String> {
    let size = till - from;
    let mut result = Vec::with_capacity(size);
    for _ in 0..size {
        result.push(liz_forms::forms_del(forms, from));
    }
    result
}

pub fn parse_put_range(forms: &mut Vec<String>, on: usize, range: Vec<String>) {
    let mut delta = 0;
    for form in range {
        liz_forms::forms_add(forms, on + delta, form);
        delta += 1;
    }
}

struct ParserHelper {
    results: Vec<String>,
    accrued: String,
}

impl ParserHelper {
    fn new() -> Self {
        Self {
            results: Vec::new(),
            accrued: String::new(),
        }
    }

    fn accrue_char(&mut self, ch: char) {
        self.accrued.push(ch);
    }

    fn commit_accrued(&mut self) {
        if !self.accrued.is_empty() {
            self.results.push(self.accrued.clone());
            self.accrued.clear();
        }
    }

    fn got_form(&mut self, from: String) {
        self.results.push(from);
    }
}
