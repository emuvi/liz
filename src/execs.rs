use crate::LizError;
use rlua::UserData;
use simple_error::SimpleError;
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct Spawned {
	results: Arc<RwLock<Option<Result<Vec<String>, LizError>>>>,
}

impl Spawned {
	fn new() -> Spawned {
		Spawned {
			results: Arc::new(RwLock::new(None)),
		}
	}

	fn join(&self) -> Result<Vec<String>, LizError> {
		loop {
			{
				let lock = self.results.read().unwrap();
				if lock.is_some() {
					break;
				}
			}
			thread::sleep(Duration::from_millis(10));
		}
		let lock = self.results.read().unwrap();
		if let Some(results) = &*lock {
			match results {
				Ok(results) => Ok(results.clone()),
				Err(err) => Err(Box::new(SimpleError::new(format!(
					"Spawned process returned error. - {}.",
					err
				)))),
			}
		} else {
			Err(Box::new(SimpleError::new(
				"Could not get the results of none.",
			)))
		}
	}
}

impl UserData for Spawned {}

pub fn spawn(path: String, args: Option<Vec<String>>) -> Spawned {
	let exec_path = if !path.ends_with(".liz") || !path.ends_with(".lua") {
		format!("{}.liz", path)
	} else {
		path
	};
	let spawned = Spawned::new();
	let spawned_clone = spawned.clone();
	thread::spawn(move || {
		let returned = crate::run(exec_path, args);
		{
			let mut lock = spawned_clone.results.write().unwrap();
			*lock = Some(returned);
		}
	});
	spawned
}

pub fn join(spawned: Spawned) -> Result<Vec<String>, LizError> {
	spawned.join()
}

pub fn cmd<S: AsRef<str>, P: AsRef<Path>>(
	name: &str,
	args: &[S],
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

pub fn pause() {
  use std::io::Write;
  let stdout = std::io::stdout();
  let mut reader = std::io::stdin();
  let mut buffer = [0;1];
  print!("Press enter to continue...");
  stdout.lock().flush().unwrap();
  reader.read_exact(&mut buffer).unwrap();
}
