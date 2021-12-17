use std::path::Path;

use crate::LizError;

use crate::execs;
use crate::files;

pub fn git_root_find(path: impl AsRef<Path>) -> Result<Option<String>, LizError> {
    let mut actual = files::path_absolute(path)?;
    loop {
        let check = files::path_join(&actual, ".git")?;
        if files::is_dir(&check) {
            return Ok(Some(actual));
        }
		actual = files::path_parent(&actual)?;
		if actual.is_empty() {
			break;
        }
    }
    Ok(None)
}

pub fn git_is_ignored(path: impl AsRef<Path>) -> Result<bool, LizError> {
	if let Some(root) = git_root_find(&path)? {
		let relative = files::path_relative(path, &root)?;
        let (code, output) = execs::cmd("git", &["check-ignore", &relative], &root, false, false)?;
		return Ok(code == 0 && !output.is_empty());
    }
    Ok(false)
}