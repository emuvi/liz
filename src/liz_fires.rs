use rlua::{Context, Table, UserData};
use simple_error::SimpleError;

use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

use crate::liz_paths;
use crate::utils::{self, dbg_err};
use crate::LizError;

#[derive(Clone)]
pub struct Spawned {
    path: String,
    args: Option<Vec<String>>,
    results: Arc<RwLock<Option<Result<Vec<String>, LizError>>>>,
}

impl Spawned {
    fn new(path: String, args: Option<Vec<String>>) -> Spawned {
        Spawned {
            path,
            args,
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

pub fn spawn(lane: Context, path: &str, args: &Option<Vec<String>>) -> Result<Spawned, LizError> {
    let globals = lane.globals();
    let liz: Table = globals.get("liz")?;

    let path = utils::add_liz_extension(path);
    let path = if liz_paths::is_relative(&path) {
        let stack_dir = utils::get_stack_dir(&liz).map_err(|err| dbg_err!(err, "get_stack_dir"))?;
        liz_paths::path_join(&stack_dir, &path)
            .map_err(|err| dbg_err!(err, "path_join", stack_dir, path))?
    } else {
        path
    };

    let spawn_pwd = liz_paths::pwd().map_err(|err| dbg_err!(err, "pwd"))?;
    liz.set("spawn_pwd", spawn_pwd)?;

    let spawn_dir =
        liz_paths::path_parent(&path).map_err(|err| dbg_err!(err, "path_parent", path))?;
    utils::put_stack_dir(&lane, &liz, spawn_dir.clone())
        .map_err(|err| dbg_err!(err, "put_stack_dir", spawn_dir))?;
    liz.set("spawn_dir", spawn_dir)?;

    let spawn_path =
        liz_paths::path_absolute(&path).map_err(|err| dbg_err!(err, "path_absolute", path))?;
    liz.set("spawn_path", spawn_path.clone())?;

    let spawned = Spawned::new(spawn_path, args.clone());
    let spawned_clone = spawned.clone();
    thread::spawn(move || {
        let returned = crate::run(&spawned_clone.path, &spawned_clone.args);
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
    let args = args
        .iter()
        .map(|arg| {
            let arg = arg.as_ref();
            cmd.arg(arg);
            arg.into()
        })
        .collect::<Vec<&str>>();
    let dir: String = if let Some(dir) = dir {
        dir.as_ref().into()
    } else {
        ".".into()
    };
    cmd.current_dir(&dir);
    let mut child = cmd
        .stdin(Stdio::null())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|err| dbg_err!(err, "spawn", name, args, dir))?;
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

pub fn exe_ext() -> &'static str {
    std::env::consts::EXE_EXTENSION
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
