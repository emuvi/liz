use crate::liz_debug::dbg_bleb;
use crate::liz_debug::{dbg_call, dbg_reav, dbg_seal, dbg_tell};
use crate::liz_texts;
use crate::LizError;

#[derive(Debug, Clone, PartialEq)]
pub struct Forms {
    pub desk: Vec<String>,
}

pub fn forms_len(forms: &Vec<String>) -> usize {
    dbg_call!();
    dbg_reav!(forms.len());
}

pub fn forms_get(forms: &Vec<String>, index: usize) -> &str {
    dbg_call!(index);
    dbg_reav!(&forms[index]);
}

pub fn forms_set(forms: &mut Vec<String>, index: usize, form: String) {
    dbg_call!(index, form);
    forms[index] = form;
}

pub fn forms_add(forms: &mut Vec<String>, index: usize, form: String) {
    dbg_call!(index, form);
    forms.insert(index, form)
}

pub fn forms_put(forms: &mut Vec<String>, form: String) {
    dbg_call!(form);
    forms.push(form)
}

pub fn forms_del(forms: &mut Vec<String>, index: usize) -> String {
    dbg_call!(index);
    dbg_reav!(forms.remove(index));
}

pub fn forms_pop(forms: &mut Vec<String>) -> Option<String> {
    dbg_call!();
    dbg_reav!(forms.pop());
}

pub fn forms_find_all(forms: &Vec<String>, part: &str) -> Vec<usize> {
    dbg_call!(part);
    dbg_reav!(forms_find_all_ask(forms, |form| {
        form_is_equals(form, part)
    }));
}

pub fn forms_find_all_like(forms: &Vec<String>, part: &str) -> Vec<usize> {
    dbg_call!(part);
    dbg_reav!(forms_find_all_ask(forms, |form| {
        form_is_likely(form, part)
    }));
}

pub fn forms_find_all_ask<F: Fn(&str) -> bool>(forms: &Vec<String>, ask: F) -> Vec<usize> {
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

pub fn forms_first_some(forms: &Vec<String>) -> Option<usize> {
    dbg_call!();
    let mut index = 0;
    while index < forms.len() {
        if !form_is_whitespace(&forms[index]) {
            dbg_reav!(Some(index));
        }
        index += 1;
    }
    dbg_reav!(None);
}

pub fn forms_prior_some(forms: &Vec<String>, of: usize) -> Option<usize> {
    dbg_call!();
    let mut index = of;
    while index > 0 {
        index -= 1;
        if !form_is_whitespace(&forms[index]) {
            dbg_reav!(Some(index));
        }
    }
    dbg_reav!(None);
}

pub fn forms_later_some(forms: &Vec<String>, of: usize) -> Option<usize> {
    dbg_call!();
    let mut index = of;
    while index < forms.len() - 1 {
        index += 1;
        if !form_is_whitespace(&forms[index]) {
            dbg_reav!(Some(index));
        }
    }
    dbg_reav!(None);
}

pub fn forms_final_some(forms: &Vec<String>) -> Option<usize> {
    dbg_call!();
    let mut index = forms.len();
    while index > 0 {
        index -= 1;
        if !form_is_whitespace(&forms[index]) {
            dbg_reav!(Some(index));
        }
    }
    dbg_reav!(None);
}

pub fn forms_change_all(forms: &mut Vec<String>, of: &str, to: &str) {
    dbg_call!(of, to);
    forms_change_all_ask(forms, of, to, |form| form_is_equals(form, of))
}

pub fn forms_change_all_like(forms: &mut Vec<String>, of: &str, to: &str) {
    dbg_call!(of, to);
    forms_change_all_ask(forms, of, to, |form| form_is_likely(form, of))
}

pub fn forms_change_all_ask<F: Fn(&str) -> bool>(
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

pub fn forms_print(forms: &Vec<String>, index: usize) {
    dbg_call!();
    form_print(&forms[index]);
}

pub fn forms_println(forms: &Vec<String>, index: usize) {
    dbg_call!();
    form_println(&forms[index]);
}

pub fn forms_print_all(forms: &Vec<String>) {
    dbg_call!();
    print!("[");
    let mut first = true;
    for form in forms {
        if first {
            first = false;
        } else {
            print!(",")
        }
        form_print(form);
    }
    println!("]");
}

pub fn forms_build(forms: &Vec<String>) -> String {
    dbg_call!();
    let mut result = String::new();
    for form in forms {
        result.push_str(form);
    }
    dbg_reav!(result);
}

pub fn forms_write(forms: &Vec<String>, path: &str) -> Result<(), LizError> {
    dbg_call!(path);
    let contents = forms_build(forms);
    dbg_seal!(contents);
    liz_texts::write(path, contents).map_err(|err| dbg_bleb!(err))
}

pub fn form_is_equals(form: &str, with: &str) -> bool {
    dbg_call!(form, with);
    dbg_reav!(form == with);
}

pub fn form_is_likely(form: &str, with: &str) -> bool {
    dbg_call!(form, with);
    dbg_reav!(liz_texts::is_likely(form, with));
}

pub fn form_print(form: &str) {
    dbg_call!();
    print!("'{}'", form);
}

pub fn form_println(form: &str) {
    dbg_call!();
    println!("'{}'", form);
}

pub fn form_is_whitespace(form: &str) -> bool {
    dbg_call!();
    dbg_reav!(!form.chars().any(|ch| !ch.is_whitespace()));
}

pub fn form_is_linespace(form: &str) -> bool {
    dbg_call!();
    dbg_reav!(!form
        .chars()
        .any(|ch| LINE_SPACE_CHARS.iter().any(|item| ch != *item)));
}

pub fn form_is_linebreak(form: &str) -> bool {
    dbg_call!();
    dbg_reav!(!form
        .chars()
        .any(|ch| LINE_BREAK_CHARS.iter().any(|item| ch != *item)));
}

pub fn form_is_brackets(form: &str) -> bool {
    dbg_call!();
    dbg_reav!(!form
        .chars()
        .any(|ch| BRACKETS_CHARS.iter().any(|item| ch != *item)));
}

pub fn form_is_quotation(form: &str) -> bool {
    dbg_call!();
    dbg_reav!(!form
        .chars()
        .any(|ch| QUOTATION_CHARS.iter().any(|item| ch != *item)));
}

pub static LINE_SPACE_CHARS: &[char] = &[' ', '\t'];

pub static LINE_BREAK_CHARS: &[char] = &['\n', '\r'];

pub static BRACKETS_CHARS: &[char] = &['(', ')', '[', ']', '{', '}', '<', '>'];

pub static QUOTATION_CHARS: &[char] = &['\'', '"'];
