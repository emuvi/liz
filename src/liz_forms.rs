use crate::liz_debug::dbg_bleb;
use crate::liz_debug::{dbg_call, dbg_reav, dbg_step, dbg_tell};
use crate::liz_texts;
use crate::LizError;

#[derive(Debug, Clone, PartialEq)]
pub struct Forms {
    pub desk: Vec<String>,
}

pub fn kit_new(from: &[impl AsRef<str>]) -> Vec<String> {
    let mut result = Vec::with_capacity(from.len());
    for item in from {
        result.push(item.as_ref().into());
    }
    result
}

pub fn kit_len(forms: &Vec<String>) -> usize {
    dbg_call!();
    dbg_reav!(forms.len());
}

pub fn kit_get(forms: &Vec<String>, index: usize) -> &str {
    dbg_call!(index);
    dbg_reav!(&forms[index]);
}

pub fn kit_set(forms: &mut Vec<String>, index: usize, form: String) {
    dbg_call!(index, form);
    forms[index] = form;
}

pub fn kit_add(forms: &mut Vec<String>, index: usize, form: String) {
    dbg_call!(index, form);
    forms.insert(index, form)
}

pub fn kit_put(forms: &mut Vec<String>, form: String) {
    dbg_call!(form);
    forms.push(form)
}

pub fn kit_del(forms: &mut Vec<String>, index: usize) -> String {
    dbg_call!(index);
    dbg_reav!(forms.remove(index));
}

pub fn kit_pop(forms: &mut Vec<String>) -> Option<String> {
    dbg_call!();
    dbg_reav!(forms.pop());
}

pub fn kit_find_all(forms: &Vec<String>, part: &str) -> Vec<usize> {
    dbg_call!(part);
    dbg_reav!(kit_find_all_ask(forms, |form| {
        liz_texts::is_equals(form, part)
    }));
}

pub fn kit_find_all_like(forms: &Vec<String>, part: &str) -> Vec<usize> {
    dbg_call!(part);
    dbg_reav!(kit_find_all_ask(forms, |form| {
        liz_texts::is_likely(form, part)
    }));
}

pub fn kit_find_all_ask<F: Fn(&str) -> bool>(forms: &Vec<String>, ask: F) -> Vec<usize> {
    dbg_call!();
    let mut result = Vec::new();
    let mut index = 0;
    for form in forms {
        if ask(form) {
            dbg_tell!(form, index);
            result.push(index);
        }
        index += 1;
    }
    dbg_reav!(result);
}

pub fn kit_first_some(forms: &Vec<String>) -> Option<usize> {
    dbg_call!();
    let mut index = 0;
    while index < forms.len() {
        if !liz_texts::is_whitespace(&forms[index]) {
            dbg_reav!(Some(index));
        }
        index += 1;
    }
    dbg_reav!(None);
}

pub fn kit_prior_some(forms: &Vec<String>, of: usize) -> Option<usize> {
    dbg_call!();
    let mut index = of;
    while index > 0 {
        index -= 1;
        if !liz_texts::is_whitespace(&forms[index]) {
            dbg_reav!(Some(index));
        }
    }
    dbg_reav!(None);
}

pub fn kit_next_some(forms: &Vec<String>, of: usize) -> Option<usize> {
    dbg_call!();
    let mut index = of;
    while index < forms.len() - 1 {
        index += 1;
        if !liz_texts::is_whitespace(&forms[index]) {
            dbg_reav!(Some(index));
        }
    }
    dbg_reav!(None);
}

pub fn kit_last_some(forms: &Vec<String>) -> Option<usize> {
    dbg_call!();
    let mut index = forms.len();
    while index > 0 {
        index -= 1;
        if !liz_texts::is_whitespace(&forms[index]) {
            dbg_reav!(Some(index));
        }
    }
    dbg_reav!(None);
}

pub fn kit_change_all(forms: &mut Vec<String>, of: &str, to: &str) {
    dbg_call!(of, to);
    kit_change_all_ask(forms, of, to, |form| liz_texts::is_equals(form, of))
}

pub fn kit_change_all_like(forms: &mut Vec<String>, of: &str, to: &str) {
    dbg_call!(of, to);
    kit_change_all_ask(forms, of, to, |form| liz_texts::is_likely(form, of))
}

pub fn kit_change_all_ask<F: Fn(&str) -> bool>(
    forms: &mut Vec<String>,
    of: &str,
    to: &str,
    ask: F,
) {
    dbg_call!(of, to);
    let mut indexes = Vec::new();
    for (index, form) in forms.iter().enumerate() {
        if ask(form) {
            indexes.push(index);
        }
    }
    for index in indexes {
        forms[index] = to.into();
    }
}

pub fn kit_print_all(forms: &Vec<String>) {
    dbg_call!();
    print!("[");
    let mut first = true;
    for form in forms {
        if first {
            first = false;
        } else {
            print!(",")
        }
        print!("'{}'", form);
    }
    println!("]");
}

pub fn kit_build(forms: &Vec<String>) -> String {
    dbg_call!();
    let mut result = String::new();
    for form in forms {
        result.push_str(form);
    }
    dbg_reav!(result);
}

pub fn kit_write(forms: &Vec<String>, path: &str) -> Result<(), LizError> {
    dbg_call!(path);
    let contents = kit_build(forms);
    dbg_step!(contents);
    liz_texts::write(path, contents).map_err(|err| dbg_bleb!(err))
}
