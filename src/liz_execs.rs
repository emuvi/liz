use rlua::{Context, Table, UserData};
use simple_error::SimpleError;

use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

use crate::liz_files;
use crate::utils;
use crate::LizError;

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

pub fn spawn(ctx: Context, path: &str, args: Option<Vec<String>>) -> Result<Spawned, LizError> {
    let globals = ctx.globals();
    let liz: Table = globals.get("liz")?;

    let path = utils::add_liz_extension(path);
    let path = if liz_files::is_relative(&path) {
        let stack_dir = utils::get_stack_dir(&liz)?;
        liz_files::path_join(&stack_dir, &path)?
    } else {
        path
    };

    let spawn_pwd = liz_files::pwd()?;
    liz.set("spawn_pwd", spawn_pwd)?;

    let spawn_dir = liz_files::path_parent(&path)?;
    utils::put_stack_dir(&ctx, &liz, spawn_dir.clone())?;
    liz.set("spawn_dir", spawn_dir)?;

    let spawn_path = liz_files::path_absolute(&path)?;
    liz.set("spawn_path", spawn_path.clone())?;

    let spawned = Spawned::new();
    let spawned_clone = spawned.clone();
    thread::spawn(move || {
        let returned = crate::run(&spawn_path, args);
        {
            let mut lock = spawned_clone.results.write().unwrap();
            *lock = Some(returned);
        }
    });
    Ok(spawned)
}

pub fn join(spawned: Spawned) -> Result<Vec<String>, LizError> {
    spawned.join()
}

pub fn cmd(
    name: &str,
    args: &[impl AsRef<str>],
    dir: Option<impl AsRef<str>>,
    print: Option<bool>,
    throw: Option<bool>,
) -> Result<(i32, String), LizError> {
    let mut cmd = Command::new(name);
    for arg in args {
        cmd.arg(arg.as_ref());
    }
    let dir: String = if let Some(dir) = dir {
        dir.as_ref().into()
    } else {
        ".".into()
    };
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
    let print = if let Some(print) = print { print } else { true };
    if print && !out.is_empty() {
        println!("{}", out);
    }
    let throw = if let Some(throw) = throw { throw } else { true };
    if throw && res != 0 {
        return Err(Box::new(SimpleError::new(format!(
            "Exit code from {} command is different than zero: {}.",
            name, res
        ))));
    }
    Ok((res, out))
}

pub fn pause() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();
    let mut buffer = [0u8];
    stdin.read(&mut buffer).unwrap();
    if !(buffer[0] == b'\r' || buffer[0] == b'\n') {
        write!(stdout, "\n").unwrap();
        stdout.flush().unwrap();
    }
}

pub fn get_os() -> &'static str {
    std::env::consts::OS
}

pub fn is_lin() -> bool {
    std::env::consts::OS == "linux"
}

pub fn is_mac() -> bool {
    std::env::consts::OS == "macos"
}

pub fn is_win() -> bool {
    std::env::consts::OS == "windows"
}
