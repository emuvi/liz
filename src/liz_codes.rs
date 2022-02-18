use rlua::{UserData, UserDataMethods};

use crate::liz_execs;
use crate::liz_forms::{Form, Forms};
use crate::liz_parse::{Parser, CODE_PARSER};
use crate::liz_paths;
use crate::liz_texts;
use crate::utils;
use crate::LizError;

impl UserData for Forms {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("len", |_, var, ()| Ok(var.len()));
        methods.add_method("get", |_, var, index: usize| Ok(var.get(index).clone()));
        methods.add_method_mut("set", |_, var, (index, form): (usize, Form)| {
            Ok(var.set(index, form))
        });
        methods.add_method_mut("add", |_, var, (index, form): (usize, Form)| {
            Ok(var.add(index, form))
        });
        methods.add_method_mut("put", |_, var, form: Form| Ok(var.put(form)));
        methods.add_method_mut("del", |_, var, index: usize| Ok(var.del(index)));
        methods.add_method_mut("pop", |_, var, ()| Ok(var.pop()));
        methods.add_method_mut("change_all", |_, var, (of, to): (String, String)| {
            Ok(var.change_all(&of, &to))
        });
        methods.add_method("build", |_, var, ()| Ok(var.build()));
        methods.add_method("write", |lane, var, path: String| {
            let text = var.build();
            utils::treat_error(lane, liz_texts::write(&path, &text))
        });
    }
}

impl UserData for Form {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("is_whitespace", |_, var, ()| Ok(var.is_whitespace()));
        methods.add_method("is_linespace", |_, var, ()| Ok(var.is_linespace()));
        methods.add_method("is_linebreak", |_, var, ()| Ok(var.is_linebreak()));
        methods.add_method("is_code_brackets", |_, var, ()| Ok(var.is_code_brackets()));
        methods.add_method("is_text_brackets", |_, var, ()| Ok(var.is_text_brackets()));
        methods.add_method(
            "is_text_quotation",
            |_, var, ()| Ok(var.is_text_quotation()),
        );
    }
}

pub fn edit() -> Forms {
    Forms::edit()
}

pub fn code(source: &str) -> Forms {
    CODE_PARSER.parse(source)
}

pub fn form(part: &str) -> Form {
    Form::new(part)
}

pub fn git_root_find(path: &str) -> Result<Option<String>, LizError> {
    let mut actual = liz_paths::path_absolute(path)?;
    loop {
        let check = liz_paths::path_join(&actual, ".git")?;
        if liz_paths::is_dir(&check) {
            return Ok(Some(actual));
        }
        actual = liz_paths::path_parent(&actual)?;
        if actual.is_empty() {
            break;
        }
    }
    Ok(None)
}

pub fn git_is_ignored(path: &str) -> Result<bool, LizError> {
    if let Some(root) = git_root_find(path)? {
        let relative = liz_paths::path_relative(path, &root)?;
        let (code, output) = liz_execs::cmd(
            "git",
            &["check-ignore", &relative],
            Some(&root),
            Some(false),
            Some(false),
        )?;
        return Ok(code == 0 && !output.is_empty());
    }
    Ok(false)
}

pub fn git_has_changes(root: &str) -> Result<bool, LizError> {
    let (_, output) = liz_execs::cmd("git", &["status"], Some(root), Some(false), Some(true))?;
    let output = output.trim();
    Ok(!output.ends_with("nothing to commit, working tree clean"))
}
