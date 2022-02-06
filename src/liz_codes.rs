use rlua::{UserData, UserDataMethods};

use crate::liz_execs;
use crate::liz_files;
use crate::liz_slabs;
use crate::liz_slabs::Slabs;
use crate::liz_texts;
use crate::utils;
use crate::LizError;

#[derive(Clone)]
pub struct Source {
    path: String,
    slabs: Slabs,
}

pub fn source(path: &str) -> Result<Source, LizError> {
    let path = String::from(path);
    let name = liz_files::path_name(&path);
    let text = if liz_files::is_file(&path) {
        liz_texts::read(&path)?
    } else {
        String::new()
    };
    let slabs = liz_slabs::parse(&text, name);
    Ok(Source { path, slabs })
}

impl UserData for Source {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("put", |_, src, part: String| {
            src.slabs.put(&part);
            Ok(())
        });

        methods.add_method("len", |_, src, ()| Ok(src.slabs.len()));

        methods.add_method("get", |_, src, index: usize| Ok(src.slabs.get(index)));

        methods.add_method("build", |_, src, ()| Ok(src.slabs.build()));

        methods.add_method("write", |ctx, src, ()| {
            let text = src.slabs.build();
            utils::treat_error(ctx, liz_texts::write(&src.path, &text))
        });
    }
}

pub fn git_root_find(path: &str) -> Result<Option<String>, LizError> {
    let mut actual = liz_files::path_absolute(path)?;
    loop {
        let check = liz_files::path_join(&actual, ".git")?;
        if liz_files::is_dir(&check) {
            return Ok(Some(actual));
        }
        actual = liz_files::path_parent(&actual)?;
        if actual.is_empty() {
            break;
        }
    }
    Ok(None)
}

pub fn git_is_ignored(path: &str) -> Result<bool, LizError> {
    if let Some(root) = git_root_find(path)? {
        let relative = liz_files::path_relative(path, &root)?;
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
