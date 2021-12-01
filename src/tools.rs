use simple_error::SimpleError;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

use crate::LizError;

pub fn cmd<A: AsRef<str>, P: AsRef<Path>>(
	name: &str,
	args: &[A],
	dir: P,
	print: bool,
	throw: bool,
) -> Result<(i32, String), LizError> {
	let mut cmd = Command::new(name);
	for arg in args {
		cmd.arg(arg.as_ref());
	}
	cmd.current_dir(dir);
	let mut child = cmd
		.stdin(Stdio::null())
		.stderr(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()?;
	let mut out = String::new();
	child.stderr.take().unwrap().read_to_string(&mut out)?;
	child.stdout.take().unwrap().read_to_string(&mut out)?;
	let out = out.trim();
	let out = String::from(out);
	let res = child.wait()?.code().unwrap_or(0);
	if print && !out.is_empty() {
		println!("{}", out);
	}
	if throw && res != 0 {
		return Err(Box::new(SimpleError::new(format!(
			"Exit code from {} command is different than zero: {}.",
			name, res
		))));
	}
	Ok((res, out))
}

pub fn has(path: &str) -> bool {
	Path::new(path).exists()
}

pub fn isdir(path: &str) -> bool {
	Path::new(path).is_dir()
}

pub fn rn(origin: &str, destiny: &str) -> Result<(), LizError> {
	Ok(fs::rename(origin, destiny)?)
}

pub fn cp(origin: &str, destiny: &str) -> Result<(), LizError> {
	if isdir(origin) {
		copy_directory(origin, destiny)?;
	} else {
		copy_file(origin, destiny)?;
	}
	Ok(())
}

pub fn cp_tmp(origin: &str, destiny: &str) -> Result<(), LizError> {
	if has(destiny) {
		let mut temp_name: String = destiny
			.chars()
			.map(|x| match x {
				'\\' => '_',
				'/' => '_',
				_ => x,
			})
			.collect();
		let mut destiny_tmp = std::env::temp_dir().join(&temp_name);
		while destiny_tmp.exists() {
			temp_name.push_str("_");
			destiny_tmp = std::env::temp_dir().join(&temp_name);
		}
		fs::rename(destiny, destiny_tmp)?;
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

pub fn mv(origin: &str, destiny: &str) -> Result<(), LizError> {
	cp(origin, destiny)?;
	rm(origin)?;
	Ok(())
}

pub fn rm(path: &str) -> Result<(), LizError> {
	Ok(if has(path) {
		if isdir(path) {
			fs::remove_dir_all(path)?
		} else {
			fs::remove_file(path)?
		}
	})
}

pub fn read(path: &str) -> Result<String, LizError> {
	let mut file = fs::File::open(path)?;
	let mut result = String::new();
	file.read_to_string(&mut result)?;
	Ok(result)
}

pub fn mkdir(path: &str) -> Result<(), LizError> {
	fs::create_dir_all(path)?;
	Ok(())
}

pub fn touch(path: &str) -> Result<(), LizError> {
	fs::OpenOptions::new().create(true).write(true).open(path)?;
	Ok(())
}

pub fn write(path: &str, contents: &str) -> Result<(), LizError> {
	Ok(fs::write(path, contents)?)
}

pub fn append(path: &str, contents: &str) -> Result<(), LizError> {
	let mut file = fs::OpenOptions::new().write(true).append(true).open(path)?;
	Ok(writeln!(file, "{}", contents)?)
}

pub fn exe_ext() -> &'static str {
	std::env::consts::EXE_EXTENSION
}
