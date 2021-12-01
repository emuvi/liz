use simple_error::SimpleError;
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
	if print {
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

pub fn cp(origin: &str, destiny: &str) -> Result<(), LizError> {
	if std::fs::metadata(origin)?.is_dir() {
		copy_directory(origin, destiny)?;
	} else {
		copy_file(origin, destiny)?;
	}
	Ok(())
}

fn copy_directory(origin: impl AsRef<Path>, destiny: impl AsRef<Path>) -> Result<(), LizError> {
	std::fs::create_dir_all(&destiny)?;
	for entry in std::fs::read_dir(origin)? {
		let entry = entry?;
		let file_type = entry.file_type()?;
		if file_type.is_dir() {
			copy_directory(entry.path(), destiny.as_ref().join(entry.file_name()))?;
		} else {
			std::fs::copy(entry.path(), destiny.as_ref().join(entry.file_name()))?;
		}
	}
	Ok(())
}

fn copy_file(origin: impl AsRef<Path>, destiny: impl AsRef<Path>) -> Result<(), LizError> {
	if let Some(parent) = destiny.as_ref().parent() {
		std::fs::create_dir_all(parent)?;
	}
	std::fs::copy(origin, destiny)?;
	Ok(())
}

pub fn mv(origin: &str, destiny: &str) -> Result<(), LizError> {
	cp(origin, destiny)?;
	rm(origin)?;
	Ok(())
}

pub fn rm(path: &str) -> Result<(), LizError> {
	let path = std::path::Path::new(path);
	if path.exists() {
		if path.is_dir() {
			std::fs::remove_dir_all(path)?;
		} else {
			std::fs::remove_file(path)?;
		}
	}
	Ok(())
}

pub fn mk_dir(path: &str) -> Result<(), LizError> {
	std::fs::create_dir_all(path)?;
	Ok(())
}

pub fn write(path: &str, contents: &str) -> Result<(), LizError> {
	std::fs::write(path, contents)?;
	Ok(())
}

pub fn append(path: &str, contents: &str) -> Result<(), LizError> {
	let mut file = std::fs::OpenOptions::new()
		.write(true)
		.append(true)
		.open(path)?;
	writeln!(file, "{}", contents)?;
	Ok(())
}

pub fn exe_ext() -> &'static str {
	std::env::consts::EXE_EXTENSION
}
