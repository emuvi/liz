use std::path::Path;

use crate::LizError;

use crate::liz_execs;
use crate::liz_files;

pub fn git_root_find(path: impl AsRef<Path>) -> Result<Option<String>, LizError> {
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

pub fn git_is_ignored(path: impl AsRef<Path>) -> Result<bool, LizError> {
    if let Some(root) = git_root_find(&path)? {
        let relative = liz_files::path_relative(path, &root)?;
        let (code, output) =
            liz_execs::cmd("git", &["check-ignore", &relative], &root, false, false)?;
        return Ok(code == 0 && !output.is_empty());
    }
    Ok(false)
}

pub fn git_has_changes(root: impl AsRef<Path>) -> Result<bool, LizError> {
    let (_, output) = liz_execs::cmd("git", &["status"], root, false, true)?;
    let output = output.trim();
    Ok(!output.ends_with("nothing to commit, working tree clean"))
}
