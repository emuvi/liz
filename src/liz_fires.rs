use rlua::{Context, Table, UserData};

use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

use crate::liz_codes;
use crate::liz_debug::{dbg_call, dbg_reav, dbg_step};
use crate::liz_debug::{dbg_ebb, dbg_err, dbg_knd};
use crate::liz_paths;
use crate::utils;
use crate::LizError;

pub fn race_wd(lane: Context, relative_path: &str) -> Result<Vec<String>, LizError> {
    dbg_call!(relative_path);
    let working_dir = liz_paths::wd().map_err(|err| dbg_ebb!(err))?;
    dbg_step!(working_dir);
    let full_path =
        liz_paths::path_join(&working_dir, relative_path).map_err(|err| dbg_ebb!(err))?;
    dbg_step!(full_path);
    dbg_reav!(crate::race_in(lane, &full_path).map_err(|err| dbg_ebb!(err)));
}

pub fn spawn(lane: Context, path: &str, args: &Option<Vec<String>>) -> Result<Spawned, LizError> {
    dbg_call!(path, args);
    let globals = lane.globals();
    let liz: Table = globals.get("liz").map_err(|err| dbg_err!(err))?;

    let suit_path = liz_codes::liz_suit_path(path).map_err(|err| dbg_ebb!(err))?;
    dbg_step!(suit_path);

    let suit_path = if liz_paths::is_relative(&suit_path) {
        let stack_dir = utils::get_stack_dir(&liz).map_err(|err| dbg_ebb!(err))?;
        liz_paths::path_join(&stack_dir, &suit_path).map_err(|err| dbg_ebb!(err))?
    } else {
        suit_path
    };
    dbg_step!(suit_path);

    let spawn_wd = liz_paths::wd().map_err(|err| dbg_ebb!(err))?;
    dbg_step!(spawn_wd);

    let spawn_dir = liz_paths::path_parent(&suit_path).map_err(|err| dbg_ebb!(err))?;
    dbg_step!(spawn_dir);

    let spawn_path = liz_paths::path_absolute(&suit_path).map_err(|err| dbg_ebb!(err))?;
    dbg_step!(spawn_path);

    liz.set("spawn_wd", spawn_wd).map_err(|err| dbg_err!(err))?;
    liz.set("spawn_dir", spawn_dir)
        .map_err(|err| dbg_err!(err))?;
    liz.set("spawn_path", spawn_path.clone())
        .map_err(|err| dbg_err!(err))?;

    let spawn_index = SPAWN_COUNT.fetch_add(1, Ordering::SeqCst);
    let spawn_name = format!("spawn{}", spawn_index);
    dbg_step!(spawn_name);

    let spawned = Spawned::new(spawn_path, args.clone());
    let spawned_clone = spawned.clone();

    let builder = thread::Builder::new().name(spawn_name);
    builder
        .spawn(move || {
            let returned = crate::run(&spawned_clone.path, &spawned_clone.args);
            {
                let mut lock = spawned_clone
                    .results
                    .write()
                    .map_err(|err| dbg_err!(err))
                    .unwrap();
                *lock = Some(returned);
            }
        })
        .map_err(|err| dbg_err!(err))?;
    let result = Ok(spawned);
    dbg_reav!(result)
}

pub fn join(spawned: Spawned) -> Result<Vec<String>, LizError> {
    dbg_call!(spawned);
    dbg_reav!(spawned.join());
}

pub fn join_all(spawneds: Vec<Spawned>) -> Result<Vec<Vec<String>>, LizError> {
    dbg_call!(spawneds);
    let mut all_results: Vec<Vec<String>> = Vec::new();
    for spawned in spawneds {
        let spawned_result = spawned.join().map_err(|err| dbg_ebb!(err))?;
        dbg_step!(spawned_result);
        all_results.push(spawned_result);
    }
    dbg_reav!(Ok(all_results));
}

pub fn wait(spawned: Spawned) -> Result<(), LizError> {
    dbg_step!(spawned);
    dbg_reav!(spawned.wait())
}

pub fn wait_all(spawneds: Vec<Spawned>) -> Result<(), LizError> {
    dbg_step!(spawneds);
    for spawned in spawneds {
        spawned.wait().map_err(|err| dbg_err!(err))?
    }
    Ok(())
}

pub fn cmd(
    command: &str,
    args: &[impl AsRef<str>],
    dir: Option<impl AsRef<str>>,
    print: Option<bool>,
    throw: Option<bool>,
) -> Result<(i32, String), LizError> {
    dbg_step!();
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
    child
        .stderr
        .take()
        .ok_or("Could not take on the child stderr")
        .map_err(|err| dbg_err!(err))?
        .read_to_string(&mut out)
        .map_err(|err| dbg_err!(err))?;
    child
        .stdout
        .take()
        .ok_or("Could not take on the child stdout")
        .map_err(|err| dbg_err!(err))?
        .read_to_string(&mut out)
        .map_err(|err| dbg_err!(err))?;
    let out = out.trim();
    let out = String::from(out);
    let result = child
        .wait()
        .map_err(|err| dbg_err!(err))?
        .code()
        .ok_or("Could not found the exit code")
        .map_err(|err| dbg_err!(err))?;
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
    dbg_step!(millis);
    thread::sleep(Duration::from_millis(millis))
}

pub fn pause() -> Result<(), LizError> {
    dbg_step!();
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    write!(stdout, "Press enter to continue...").map_err(|err| dbg_err!(err))?;
    stdout.flush()?;
    let mut buffer = [0u8];
    stdin.read(&mut buffer).map_err(|err| dbg_err!(err))?;
    Ok(())
}

pub fn liz_dir() -> Result<String, LizError> {
    dbg_step!();
    Ok(
        liz_paths::path_parent(liz_exe().map_err(|err| dbg_err!(err))?.as_ref())
            .map_err(|err| dbg_err!(err))?,
    )
}

pub fn liz_exe() -> Result<String, LizError> {
    dbg_step!();
    Ok(format!(
        "{}",
        std::env::current_exe()
            .map_err(|err| dbg_err!(err))?
            .display(),
    ))
}

pub fn exe_ext() -> &'static str {
    dbg_step!();
    std::env::consts::EXE_EXTENSION
}

pub fn get_os() -> &'static str {
    dbg_step!();
    std::env::consts::OS
}

pub fn is_lin() -> bool {
    dbg_step!();
    std::env::consts::OS == "linux"
}

pub fn is_mac() -> bool {
    dbg_step!();
    std::env::consts::OS == "macos"
}

pub fn is_win() -> bool {
    dbg_step!();
    std::env::consts::OS == "windows"
}

static SPAWN_COUNT: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug, Clone)]
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
        let waiter = Duration::from_millis(10);
        loop {
            {
                let lock = self.results.read().map_err(|err| dbg_err!(err))?;
                if lock.is_some() {
                    break;
                }
            }
            thread::sleep(waiter);
        }
        let lock = self.results.read().map_err(|err| dbg_err!(err))?;
        if let Some(results) = &*lock {
            match results {
                Ok(results) => Ok(results.clone()),
                Err(err) => Err(dbg_err!(err)),
            }
        } else {
            dbg_knd!("WARN", "Could not get the results from the join");
            Err(dbg_err!("Could not get the results from the join"))
        }
    }

    fn wait(&self) -> Result<(), LizError> {
        loop {
            {
                let lock = self.results.read().map_err(|err| dbg_err!(err))?;
                if lock.is_some() {
                    break;
                }
            }
            thread::sleep(Duration::from_millis(10));
        }
        Ok(())
    }
}

impl UserData for Spawned {}
