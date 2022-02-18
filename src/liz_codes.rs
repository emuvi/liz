use rlua::{UserData, UserDataMethods};

use crate::liz_execs;
use crate::liz_forms::{Form, Forms};
use crate::liz_parse::{Parser, CODE_PARSER};
use crate::liz_paths;
use crate::liz_texts;
use crate::utils;
use crate::LizError;

#[derive(Clone)]
pub struct Forming {
    pub path: String,
    pub forms: Forms,
}

pub fn code(path: &str) -> Result<Forming, LizError> {
    let path = String::from(path);
    let text = if liz_paths::is_file(&path) {
        liz_texts::read(&path)?
    } else {
        String::new()
    };
    let forms = CODE_PARSER.parse(&text);
    Ok(Forming { path, forms })
}

impl UserData for Forming {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("len", |_, var, ()| Ok(var.forms.len()));
        methods.add_method("get", |_, var, index: usize| {
            Ok(var.forms.get(index).clone())
        });
        methods.add_method_mut("put", |_, var, part: String| Ok(var.forms.put(&part)));
        methods.add_method("build", |_, var, ()| Ok(var.forms.build()));
        methods.add_method("write", |lane, var, ()| {
            let text = var.forms.build();
            utils::treat_error(lane, liz_texts::write(&var.path, &text))
        });
    }
}

impl UserData for Form {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("is_whitespace", |_, var, ()| Ok(var.is_whitespace()));
        methods.add_method("is_linespace", |_, var, ()| Ok(var.is_linespace()));
        methods.add_method("is_linebreak", |_, var, ()| Ok(var.is_linebreak()));
    }
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
