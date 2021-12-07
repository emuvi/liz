use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::path::PathBuf;

use crate::LizError;

pub fn has(path: impl AsRef<Path>) -> bool {
	path.as_ref().exists()
}

pub fn is_dir(path: impl AsRef<Path>) -> bool {
	path.as_ref().is_dir()
}

pub fn is_file(path: impl AsRef<Path>) -> bool {
	path.as_ref().is_file()
}

pub fn cd(path: impl AsRef<Path>) -> Result<(), LizError> {
	Ok(std::env::set_current_dir(path)?)
}

pub fn pwd() -> Result<String, LizError> {
	Ok(format!("{}", std::env::current_dir()?.display()))
}

pub fn rn(origin: impl AsRef<Path>, destiny: impl AsRef<Path>) -> Result<(), LizError> {
	Ok(fs::rename(origin, destiny)?)
}

pub fn cp(origin: impl AsRef<Path>, destiny: impl AsRef<Path>) -> Result<(), LizError> {
	if is_dir(&origin) {
		copy_directory(origin, destiny)?;
	} else {
		copy_file(origin, destiny)?;
	}
	Ok(())
}

pub fn cp_tmp(origin: impl AsRef<Path>, destiny: impl AsRef<Path>) -> Result<(), LizError> {
	if has(&destiny) {
		let unknown = "unknown";
		let file_name = match destiny.as_ref().file_name() {
			Some(file_name) => match file_name.to_str() {
				Some(file_stem) => file_stem,
				None => unknown,
			},
			None => unknown,
		};
		let mut temp_name = String::from(file_name);
		let mut destiny_tmp = std::env::temp_dir().join(&temp_name);
		while destiny_tmp.exists() {
			temp_name.push_str("_");
			destiny_tmp = std::env::temp_dir().join(&temp_name);
		}
		cp(&destiny, &destiny_tmp)?;
		rm(&destiny)?;
	}
	cp(origin, destiny)
}

fn copy_directory(origin: impl AsRef<Path>, destiny: impl AsRef<Path>) -> Result<(), LizError> {
	fs::create_dir_all(&destiny)?;
	for entry in fs::read_dir(origin)? {
		let entry = entry?;
		let file_type = entry.file_type()?;
		if file_type.is_dir() {
			copy_directory(entry.path(), destiny.as_ref().join(entry.file_name()))?;
		} else {
			fs::copy(entry.path(), destiny.as_ref().join(entry.file_name()))?;
		}
	}
	Ok(())
}

fn copy_file(origin: impl AsRef<Path>, destiny: impl AsRef<Path>) -> Result<(), LizError> {
	if let Some(parent) = destiny.as_ref().parent() {
		fs::create_dir_all(parent)?;
	}
	fs::copy(origin, destiny)?;
	Ok(())
}

pub fn mv(origin: impl AsRef<Path>, destiny: impl AsRef<Path>) -> Result<(), LizError> {
	cp(&origin, &destiny)?;
	rm(origin)?;
	Ok(())
}

pub fn rm(path: impl AsRef<Path>) -> Result<(), LizError> {
	Ok(if has(&path) {
		if is_dir(&path) {
			fs::remove_dir_all(path)?
		} else {
			fs::remove_file(path)?
		}
	})
}

pub fn read(path: impl AsRef<Path>) -> Result<String, LizError> {
	let mut file = fs::File::open(path)?;
	let mut result = String::new();
	file.read_to_string(&mut result)?;
	Ok(result)
}

pub fn mk_dir(path: impl AsRef<Path>) -> Result<(), LizError> {
	fs::create_dir_all(path)?;
	Ok(())
}

pub fn touch(path: impl AsRef<Path>) -> Result<(), LizError> {
	fs::OpenOptions::new().create(true).write(true).open(path)?;
	Ok(())
}

pub fn write(path: impl AsRef<Path>, contents: &str) -> Result<(), LizError> {
	Ok(fs::write(path, contents)?)
}

pub fn append(path: impl AsRef<Path>, contents: &str) -> Result<(), LizError> {
	let mut file = fs::OpenOptions::new().write(true).append(true).open(path)?;
	Ok(writeln!(file, "{}", contents)?)
}

pub fn exe_ext() -> &'static str {
	std::env::consts::EXE_EXTENSION
}

pub fn path_sep() -> String {
	String::from(std::path::MAIN_SEPARATOR)
}

pub fn path_ext(path: impl AsRef<Path>) -> Result<String, LizError> {
	let path = path.as_ref();
	if let Some(path) = path.extension() {
		if let Some(path_str) = path.to_str() {
			return Ok(String::from(path_str));
		}
	}
	Ok(String::new())
}

pub fn path_name(path: impl AsRef<Path>) -> Result<String, LizError> {
	let path = path.as_ref();
	if let Some(path) = path.file_name() {
		if let Some(path_str) = path.to_str() {
			return Ok(String::from(path_str));
		}
	}
	Ok(String::new())
}

pub fn path_stem(path: impl AsRef<Path>) -> Result<String, LizError> {
	let path = path.as_ref();
	if let Some(path) = path.file_stem() {
		if let Some(path_str) = path.to_str() {
			return Ok(String::from(path_str));
		}
	}
	Ok(String::new())
}

pub fn path_parent(path: impl AsRef<Path>) -> Result<String, LizError> {
	let path = path.as_ref();
	let path = if path.exists() && path.is_relative() {
		std::fs::canonicalize(path)?
	} else {
		path.to_path_buf()
	};
	if let Some(path) = path.parent() {
		if let Some(path_str) = path.to_str() {
			let path_str = if path_str.starts_with("\\\\?\\") || path_str.starts_with("//?/") {
				&path_str[4..]
			} else {
				&path_str
			};
			return Ok(String::from(path_str));
		}
	}
	Ok(String::new())
}

pub fn path_parent_find(path: impl AsRef<Path>, with_name: &str) -> Result<String, LizError> {
	let mut path = format!("{}", PathBuf::from(path.as_ref()).display());
	loop {
		let parent = path_parent(&path)?;
		if parent.is_empty() {
			break;
		} else {
			let name = path_stem(&parent)?;
			if name == with_name {
				return Ok(parent);
			} else {
				path = parent;
			}
		}
	}
	Ok(String::new())
}

pub fn path_join(path: impl AsRef<Path>, child: &str) -> Result<String, LizError> {
	let path = path.as_ref().join(child);
	if let Some(path_str) = path.to_str() {
		return Ok(String::from(path_str));
	}
	Ok(String::new())
}

pub fn path_list(path: impl AsRef<Path>) -> Result<Vec<String>, LizError> {
	let mut result = Vec::new();
	for entry in fs::read_dir(path)? {
		if let Ok(entry) = entry {
			if let Some(path) = entry.path().to_str() {
				result.push(String::from(path));
			}
		}
	}
	return Ok(result);
}

pub fn path_list_dirs(path: impl AsRef<Path>) -> Result<Vec<String>, LizError> {
	let mut result = Vec::new();
	for entry in fs::read_dir(path)? {
		if let Ok(entry) = entry {
			let file_type = entry.file_type()?;
			if file_type.is_dir() {
				if let Some(path) = entry.path().to_str() {
					result.push(String::from(path));
				}
			}
		}
	}
	return Ok(result);
}

pub fn path_list_files(path: impl AsRef<Path>) -> Result<Vec<String>, LizError> {
	let mut result = Vec::new();
	for entry in fs::read_dir(path)? {
		if let Ok(entry) = entry {
			let file_type = entry.file_type()?;
			if file_type.is_file() {
				if let Some(path) = entry.path().to_str() {
					result.push(String::from(path));
				}
			}
		}
	}
	return Ok(result);
}
