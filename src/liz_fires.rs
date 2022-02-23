use rlua::{Context, Table, UserData};

use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::liz_debug::dbg_err;
use crate::liz_paths;
use crate::utils;
use crate::LizError;

#[derive(Clone)]
pub struct Spawned {
    index: usize,
    path: String,
    args: Option<Vec<String>>,
    results: Arc<RwLock<Option<Result<Vec<String>, LizError>>>>,
}

impl Spawned {
    fn new(index: usize, path: String, args: Option<Vec<String>>) -> Spawned {
        Spawned {
            index,
            path,
            args,
            results: Arc::new(RwLock::new(None)),
        }
    }

    fn join(&self) -> Result<Vec<String>, LizError> {
        let waiter = Duration::from_millis(10);
        loop {
            {
                let lock = self.results.read().unwrap();
                if lock.is_some() {
                    break;
                }
            }
            thread::sleep(waiter);
        }
        let lock = self.results.read().unwrap();
        if let Some(results) = &*lock {
            match results {
                Ok(results) => Ok(results.clone()),
                Err(err) => Err(dbg_err!(err)),
            }
        } else {
            Err(dbg_err!("Could not get the results from the join"))
        }
    }

    fn wait(&self) {
        loop {
            {
                let lock = self.results.read().unwrap();
                if lock.is_some() {
                    break;
                }
            }
            thread::sleep(Duration::from_millis(10));
        }
    }
}

impl UserData for Spawned {}

static SPAWN_COUNT: AtomicUsize = AtomicUsize::new(1);

pub fn spawn(lane: Context, path: &str, args: &Option<Vec<String>>) -> Result<Spawned, LizError> {
    let globals = lane.globals();
    let liz: Table = globals.get("liz")?;

    let path = utils::add_liz_extension(path);
    let path = if liz_paths::is_relative(&path) {
        let stack_dir = utils::get_stack_dir(&liz).map_err(|err| dbg_err!(err, path))?;
        liz_paths::path_join(&stack_dir, &path).map_err(|err| dbg_err!(err, stack_dir, path))?
    } else {
        path
    };

    let spawn_pwd = liz_paths::pwd().map_err(|err| dbg_err!(err))?;
    liz.set("spawn_pwd", spawn_pwd)?;

    let spawn_dir = liz_paths::path_parent(&path).map_err(|err| dbg_err!(err, path))?;
    utils::put_stack_dir(&lane, &liz, spawn_dir.clone()).map_err(|err| dbg_err!(err, spawn_dir))?;
    liz.set("spawn_dir", spawn_dir)?;

    let spawn_path = liz_paths::path_absolute(&path).map_err(|err| dbg_err!(err, path))?;
    liz.set("spawn_path", spawn_path.clone())?;

    let spawn_index = SPAWN_COUNT.fetch_add(1, Ordering::SeqCst);
    let spawn_name = format!("spawn{}", spawn_index);

    let spawned = Spawned::new(spawn_index, spawn_path, args.clone());
    let spawned_clone = spawned.clone();

    let builder = thread::Builder::new().name(spawn_name);
    builder.spawn(move || {
        let returned = crate::run(&spawned_clone.path, &spawned_clone.args);
        {
            let mut lock = spawned_clone.results.write().unwrap();
            *lock = Some(returned);
        }
    }).unwrap();
    Ok(spawned)
}

pub fn join(spawned: Spawned) -> Result<Vec<String>, LizError> {
    spawned.join()
}

pub fn join_all(spawneds: Vec<Spawned>) -> Result<Vec<Vec<String>>, LizError> {
    let mut result: Vec<Vec<String>> = Vec::new();
    for spawned in spawneds {
        result.push(spawned.join()?);
    }
    Ok(result)
}

pub fn wait(spawned: Spawned) {
    spawned.wait()
}

pub fn wait_all(spawneds: Vec<Spawned>) {
    for spawned in spawneds {
        spawned.wait()
    }
}

pub fn cmd(
    command: &str,
    args: &[impl AsRef<str>],
    dir: Option<impl AsRef<str>>,
    print: Option<bool>,
    throw: Option<bool>,
) -> Result<(i32, String), LizError> {
    let mut cmd = Command::new(command);
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
        .map_err(|err| dbg_err!(err, command, args, dir))?;
    let mut out = String::new();
    child.stderr.take().unwrap().read_to_string(&mut out)?;
    child.stdout.take().unwrap().read_to_string(&mut out)?;
    let out = out.trim();
    let out = String::from(out);
    let result = child.wait()?.code().unwrap_or(0);
    let print = if let Some(print) = print { print } else { true };
    if print && !out.is_empty() {
        println!("{}", out);
    }
    let throw = if let Some(throw) = throw { throw } else { true };
    if throw && result != 0 {
        return Err(dbg_err!(
            "Result code from command is different than zero",
            command,
            result
        ));
    }
    Ok((result, out))
}

pub fn sleep(millis: u64) {
    thread::sleep(Duration::from_millis(millis))
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
