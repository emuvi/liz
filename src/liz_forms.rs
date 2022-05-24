use rubx::rux_dbg_bleb;
use rubx::rux_texts;
use rubx::{rux_dbg_call, rux_dbg_reav, rux_dbg_step, rux_dbg_tell};

use crate::LizError;

#[derive(Debug, Clone)]
pub struct Forms {
  pub desk: Vec<String>,
}

pub fn kit_new() -> Vec<String> {
  Vec::new()
}

pub fn kit_from(from: &[impl AsRef<str>]) -> Vec<String> {
  let mut result = Vec::with_capacity(from.len());
  for item in from {
    result.push(item.as_ref().into());
  }
  result
}

pub fn kit_len(forms: &Vec<String>) -> usize {
  rux_dbg_call!();
  rux_dbg_reav!(forms.len());
}

pub fn kit_get(forms: &Vec<String>, index: usize) -> &str {
  rux_dbg_call!(index);
  rux_dbg_reav!(&forms[index]);
}

pub fn kit_set(forms: &mut Vec<String>, index: usize, form: String) {
  rux_dbg_call!(index, form);
  forms[index] = form;
}

pub fn kit_add(forms: &mut Vec<String>, index: usize, form: String) {
  rux_dbg_call!(index, form);
  forms.insert(index, form)
}

pub fn kit_add_range(forms: &mut Vec<String>, on: usize, range: Vec<String>) {
  let mut delta = 0;
  for form in range {
    kit_add(forms, on + delta, form);
    delta += 1;
  }
}

pub fn kit_put(forms: &mut Vec<String>, form: String) {
  rux_dbg_call!(form);
  forms.push(form)
}

pub fn kit_del(forms: &mut Vec<String>, index: usize) -> String {
  rux_dbg_call!(index);
  rux_dbg_reav!(forms.remove(index));
}

pub fn kit_del_range(forms: &mut Vec<String>, from: usize, till: usize) -> Vec<String> {
  let size = till - from;
  let mut result = Vec::with_capacity(size);
  for _ in 0..size {
    result.push(kit_del(forms, from));
  }
  result
}

pub fn kit_pop(forms: &mut Vec<String>) -> Option<String> {
  rux_dbg_call!();
  rux_dbg_reav!(forms.pop());
}

pub fn kit_find_all(forms: &Vec<String>, part: &str) -> Vec<usize> {
  rux_dbg_call!(part);
  rux_dbg_reav!(kit_find_all_ask(forms, |form| {
    rux_texts::is_equals(form, part)
  }));
}

pub fn kit_find_all_like(forms: &Vec<String>, part: &str) -> Vec<usize> {
  rux_dbg_call!(part);
  rux_dbg_reav!(kit_find_all_ask(forms, |form| {
    rux_texts::is_likely(form, part)
  }));
}

pub fn kit_find_all_ask<F: Fn(&str) -> bool>(forms: &Vec<String>, ask: F) -> Vec<usize> {
  rux_dbg_call!();
  let mut result = Vec::new();
  let mut index = 0;
  for form in forms {
    if ask(form) {
      rux_dbg_tell!(form, index);
      result.push(index);
    }
    index += 1;
  }
  rux_dbg_reav!(result);
}

pub fn kit_first_some(forms: &Vec<String>) -> Option<usize> {
  rux_dbg_call!();
  let mut index = 0;
  while index < forms.len() {
    if !rux_texts::is_whitespace(&forms[index]) {
      rux_dbg_reav!(Some(index));
    }
    index += 1;
  }
  rux_dbg_reav!(None);
}

pub fn kit_prior_some(forms: &Vec<String>, of: usize) -> Option<usize> {
  rux_dbg_call!();
  let mut index = of;
  while index > 0 {
    index -= 1;
    if !rux_texts::is_whitespace(&forms[index]) {
      rux_dbg_reav!(Some(index));
    }
  }
  rux_dbg_reav!(None);
}

pub fn kit_next_some(forms: &Vec<String>, of: usize) -> Option<usize> {
  rux_dbg_call!();
  let mut index = of;
  while index < forms.len() - 1 {
    index += 1;
    if !rux_texts::is_whitespace(&forms[index]) {
      rux_dbg_reav!(Some(index));
    }
  }
  rux_dbg_reav!(None);
}

pub fn kit_last_some(forms: &Vec<String>) -> Option<usize> {
  rux_dbg_call!();
  let mut index = forms.len();
  while index > 0 {
    index -= 1;
    if !rux_texts::is_whitespace(&forms[index]) {
      rux_dbg_reav!(Some(index));
    }
  }
  rux_dbg_reav!(None);
}

pub fn kit_change_all(forms: &mut Vec<String>, of: &str, to: &str) {
  rux_dbg_call!(of, to);
  kit_change_all_ask(forms, to, |form| rux_texts::is_equals(form, of))
}

pub fn kit_change_all_like(forms: &mut Vec<String>, of: &str, to: &str) {
  rux_dbg_call!(of, to);
  kit_change_all_ask(forms, to, |form| rux_texts::is_likely(form, of))
}

pub fn kit_change_all_ask<F: Fn(&str) -> bool>(forms: &mut Vec<String>, to: &str, ask: F) {
  rux_dbg_call!(to);
  let mut indexes = Vec::new();
  for (index, form) in forms.iter().enumerate() {
    if ask(form) {
      indexes.push(index);
    }
  }
  for index in indexes {
    forms[index] = to.into();
  }
  rux_dbg_reav!(());
}

pub fn kit_print_all(forms: &Vec<String>) {
  rux_dbg_call!();
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
  rux_dbg_call!();
  let mut result = String::new();
  for form in forms {
    result.push_str(form);
  }
  rux_dbg_reav!(result);
}

pub fn kit_write(forms: &Vec<String>, path: &str) -> Result<(), LizError> {
  rux_dbg_call!(path);
  let contents = kit_build(forms);
  rux_dbg_step!(contents);
  rux_texts::write(path, contents).map_err(|err| rux_dbg_bleb!(err))
}
