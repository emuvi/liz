use rlua::{UserData, UserDataMethods};

use crate::liz_execs;
use crate::liz_forms::Forms;
use crate::liz_parse;
use crate::liz_paths;
use crate::liz_texts;
use crate::utils;
use crate::LizError;

#[derive(Clone)]
pub struct Source {
    pub path: String,
    pub forms: Forms,
}

pub fn source(path: &str) -> Result<Source, LizError> {
    let path = String::from(path);
    let text = if liz_paths::is_file(&path) {
        liz_texts::read(&path)?
    } else {
        String::new()
    };
    let forms = Forms::parse(&text, &liz_parse::DEFAULT_PARSER);
    Ok(Source { path, forms })
}

impl UserData for Source {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("put", |_, src, part: String| {
            src.forms.put(&part);
            Ok(())
        });

        methods.add_method("len", |_, src, ()| Ok(src.forms.len()));

        methods.add_method("get", |_, src, index: usize| Ok(String::from(src.forms.get(index))));

        methods.add_method("build", |_, src, ()| Ok(src.forms.build()));

        methods.add_method("write", |lane, src, ()| {
            let text = src.forms.build();
            utils::treat_error(lane, liz_texts::write(&src.path, &text))
        });
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
