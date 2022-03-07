use crate::liz_debug::{dbg_call, dbg_reav, dbg_step, dbg_tell};
use crate::liz_forms;

pub fn rig_split_whitespace(forms: &mut Vec<String>) -> usize {
    dbg_call!(forms);
    dbg_reav!(rig_split_whitespace_on(forms, 0, liz_forms::kit_len(forms)));
}

pub fn rig_split_whitespace_on(forms: &mut Vec<String>, from: usize, till: usize) -> usize {
    dbg_call!(forms, from, till);
    dbg_reav!(rig_split_near_ask_on(forms, from, till, |ch| ch.is_whitespace()));
}

pub fn rig_split_punctuation(forms: &mut Vec<String>) -> usize {
    dbg_call!(forms);
    dbg_reav!(rig_split_punctuation_on(forms, 0, liz_forms::kit_len(forms)));
}

pub fn rig_split_punctuation_on(forms: &mut Vec<String>, from: usize, till: usize) -> usize {
    dbg_call!(forms, from, till);
    dbg_reav!(rig_split_each_ask_on(forms, from, till, |ch| ch.is_ascii_punctuation()));
}

pub fn rig_group_whitespace(forms: &mut Vec<String>) -> usize {
    dbg_call!(forms);
    dbg_reav!(rig_group_whitespace_on(forms, 0, liz_forms::kit_len(forms)));
}

pub fn rig_group_whitespace_on(forms: &mut Vec<String>, from: usize, till: usize) -> usize {
    dbg_call!(forms, from, till);
    dbg_reav!(rig_group_near_ask_on(forms, from, till, |ch| ch.is_whitespace()));
}

pub fn rig_group_punctuation(forms: &mut Vec<String>) -> usize {
    dbg_call!(forms);
    dbg_reav!(rig_group_punctuation_on(forms, 0, liz_forms::kit_len(forms)));
}

pub fn rig_group_punctuation_on(forms: &mut Vec<String>, from: usize, till: usize) -> usize {
    dbg_call!(forms, from, till);
    dbg_reav!(rig_group_each_ask_on(forms, from, till, |ch| ch.is_ascii_punctuation()));
}

pub fn rig_split_near_ask_on<F: Fn(char) -> bool>(
    forms: &mut Vec<String>,
    from: usize,
    till: usize,
    ask: F,
) -> usize {
    dbg_call!(forms, from, till);
    let range = liz_forms::kit_del_range(forms, from, till);
    dbg_step!(range);
    let mut helps = ParserHelper::new();
    let mut state = false;
    for form in range {
        dbg_tell!(form);
        helps.commit_accrued();
        for ch in form.chars() {
            if ask(ch) != state {
                helps.commit_accrued();
                state = !state;
            }
            helps.accrue_char(ch);
        }
    }
    helps.commit_accrued();
    let results = helps.results;
    dbg_step!(results);
    let result = results.len();
    liz_forms::kit_add_range(forms, from, results);
    dbg_reav!(result);
}

pub fn rig_split_each_ask_on<F: Fn(char) -> bool>(
    forms: &mut Vec<String>,
    from: usize,
    till: usize,
    ask: F,
) -> usize {
    dbg_call!(forms, from, till);
    let range = liz_forms::kit_del_range(forms, from, till);
    dbg_step!(range);
    let mut helps = ParserHelper::new();
    for form in range {
        dbg_tell!(form);
        helps.commit_accrued();
        for ch in form.chars() {
            if ask(ch) {
                helps.commit_accrued();
                helps.got_form(String::from(ch));
            } else {
                helps.accrue_char(ch);
            }
        }
    }
    helps.commit_accrued();
    let results = helps.results;
    dbg_step!(results);
    let result = results.len();
    liz_forms::kit_add_range(forms, from, results);
    dbg_reav!(result);
}

pub fn rig_group_near_ask_on<F: Fn(char) -> bool>(
    forms: &mut Vec<String>,
    from: usize,
    till: usize,
    ask: F,
) -> usize {
    dbg_call!(forms, from, till);
    let mut range = liz_forms::kit_del_range(forms, from, till);
    dbg_step!(range);
    let mut results = Vec::new();
    loop {
        if range.is_empty() {
            break;
        }
        let side_a = if !results.is_empty() {
            results.pop().unwrap()
        } else {
            range.remove(0)
        };
        dbg_tell!(side_a);
        if range.is_empty() {
            results.push(side_a);
        } else {
            let side_b = range.remove(0);
            dbg_tell!(side_b);
            let side_a_last = side_a.chars().last();
            dbg_tell!(side_a_last);
            let side_b_first = side_b.chars().next();
            dbg_tell!(side_b_first);
            let mut should_group = false;
            if let Some(side_a_last) = side_a_last {
                if let Some(side_b_first) = side_b_first {
                    if ask(side_a_last) != ask(side_b_first) {
                        should_group = true;
                    }
                }
            }
            dbg_tell!(should_group);
            if should_group {
                let grouped = side_a + &side_b;
                dbg_tell!(grouped);
                results.push(grouped);
            } else {
                results.push(side_a);
                results.push(side_b);
            }
        }
    }
    dbg_step!(results);
    let result = results.len();
    liz_forms::kit_add_range(forms, from, results);
    dbg_reav!(result);
}

pub fn rig_group_each_ask_on<F: Fn(char) -> bool>(
    forms: &mut Vec<String>,
    from: usize,
    till: usize,
    ask: F,
) -> usize {
    dbg_call!(forms, from, till);
    let mut range = liz_forms::kit_del_range(forms, from, till);
    dbg_step!(range);
    let mut results = Vec::new();
    let mut last_should_group = false;
    loop {
        if range.is_empty() {
            break;
        }
        let side_a = if !results.is_empty() {
            results.pop().unwrap()
        } else {
            range.remove(0)
        };
        dbg_tell!(side_a);
        if range.is_empty() {
            results.push(side_a);
        } else {
            let side_b = range.remove(0);
            dbg_tell!(side_b);
            let mut should_group = false;
            if last_should_group {
                should_group = true;
                last_should_group = false;
            }
            if let Some((last_index, side_b_char)) = side_b.chars().enumerate().last() {
                if last_index == 0 && ask(side_b_char) {
                    should_group = true;
                    last_should_group = true;
                }
            }
            dbg_tell!(should_group);
            if should_group {
                let grouped = side_a + &side_b;
                dbg_tell!(grouped);
                results.push(grouped);
            } else {
                results.push(side_a);
                results.push(side_b);
            }
        }
    }
    dbg_step!(results);
    let result = results.len();
    liz_forms::kit_add_range(forms, from, results);
    dbg_reav!(result);
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
            let accrued = self.accrued.clone();
            dbg_tell!(accrued);
            self.results.push(accrued);
            self.accrued.clear();
        }
    }

    fn got_form(&mut self, from: String) {
        dbg_tell!(from);
        self.results.push(from);
    }
}
