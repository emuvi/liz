use rlua::{Context, Table, UserData};
use rubx::rux_paths;
use rubx::{self, rux_dbg_call, rux_dbg_reav, rux_dbg_step};
use rubx::{rux_dbg_bleb, rux_dbg_erro, rux_dbg_errs, rux_dbg_kind};

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

use crate::liz_codes;
use crate::utils;
use crate::LizError;

pub fn run_wd(relative_path: &str, args: &Option<Vec<String>>) -> Result<Vec<String>, LizError> {
    rux_dbg_call!(relative_path);
    let working_dir = rux_paths::wd().map_err(|err| rux_dbg_bleb!(err))?;
    rux_dbg_step!(working_dir);
    let full_path =
        rux_paths::path_join(&working_dir, relative_path).map_err(|err| rux_dbg_bleb!(err))?;
    rux_dbg_step!(full_path);
    rux_dbg_reav!(crate::run(&full_path, args).map_err(|err| rux_dbg_bleb!(err)));
}

pub fn race_wd(lane: Context, relative_path: &str) -> Result<Vec<String>, LizError> {
    rux_dbg_call!(relative_path);
    let working_dir = rux_paths::wd().map_err(|err| rux_dbg_bleb!(err))?;
    rux_dbg_step!(working_dir);
    let full_path =
        rux_paths::path_join(&working_dir, relative_path).map_err(|err| rux_dbg_bleb!(err))?;
    rux_dbg_step!(full_path);
    rux_dbg_reav!(crate::race_in(lane, &full_path).map_err(|err| rux_dbg_bleb!(err)));
}

pub fn spawn(lane: Context, path: &str, args: &Option<Vec<String>>) -> Result<Spawned, LizError> {
    rux_dbg_call!(path, args);
    let globals = lane.globals();
    let liz: Table = globals.get("Liz").map_err(|err| rux_dbg_erro!(err))?;

    let suit_path = liz_codes::liz_suit_path(path).map_err(|err| rux_dbg_bleb!(err))?;
    rux_dbg_step!(suit_path);

    let suit_path = if rux_paths::is_relative(&suit_path) {
        let stack_dir = utils::liz_stacked_dir(&liz).map_err(|err| rux_dbg_bleb!(err))?;
        rux_paths::path_join(&stack_dir, &suit_path).map_err(|err| rux_dbg_bleb!(err))?
    } else {
        suit_path
    };
    rux_dbg_step!(suit_path);

    let spawn_wd = rux_paths::wd().map_err(|err| rux_dbg_bleb!(err))?;
    rux_dbg_step!(spawn_wd);

    let spawn_dir = rux_paths::path_parent(&suit_path).map_err(|err| rux_dbg_bleb!(err))?;
    rux_dbg_step!(spawn_dir);

    let spawn_path = rux_paths::path_absolute(&suit_path).map_err(|err| rux_dbg_bleb!(err))?;
    rux_dbg_step!(spawn_path);

    liz.set("spawn_wd", spawn_wd)
        .map_err(|err| rux_dbg_erro!(err))?;
    liz.set("spawn_dir", spawn_dir)
        .map_err(|err| rux_dbg_erro!(err))?;
    liz.set("spawn_path", spawn_path.clone())
        .map_err(|err| rux_dbg_erro!(err))?;

    let spawn_index = SPAWN_COUNT.fetch_add(1, Ordering::SeqCst);
    let spawn_name = format!("spawn{}", spawn_index);
    rux_dbg_step!(spawn_name);

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
                    .map_err(|err| rux_dbg_erro!(err))
                    .unwrap();
                *lock = Some(returned);
            }
        })
        .map_err(|err| rux_dbg_erro!(err))?;
    let result = Ok(spawned);
    rux_dbg_reav!(result)
}

pub fn join(spawned: Spawned) -> Result<Vec<String>, LizError> {
    rux_dbg_call!(spawned);
    rux_dbg_reav!(spawned.join());
}

pub fn join_all(spawneds: Vec<Spawned>) -> Result<Vec<Vec<String>>, LizError> {
    rux_dbg_call!(spawneds);
    let mut all_results: Vec<Vec<String>> = Vec::new();
    for spawned in spawneds {
        let spawned_result = spawned.join().map_err(|err| rux_dbg_bleb!(err))?;
        rux_dbg_step!(spawned_result);
        all_results.push(spawned_result);
    }
    rux_dbg_reav!(Ok(all_results));
}

pub fn wait(spawned: Spawned) -> Result<(), LizError> {
    rux_dbg_call!(spawned);
    spawned.wait()
}

pub fn wait_all(spawneds: Vec<Spawned>) -> Result<(), LizError> {
    rux_dbg_call!(spawneds);
    for spawned in spawneds {
        spawned.wait().map_err(|err| rux_dbg_erro!(err))?
    }
    Ok(())
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
                let lock = self.results.read().map_err(|err| rux_dbg_erro!(err))?;
                if lock.is_some() {
                    break;
                }
            }
            thread::sleep(waiter);
        }
        let lock = self.results.read().map_err(|err| rux_dbg_erro!(err))?;
        if let Some(results) = &*lock {
            match results {
                Ok(results) => Ok(results.clone()),
                Err(err) => Err(rux_dbg_erro!(err)),
            }
        } else {
            rux_dbg_kind!("WARN", "Could not get the results from the join");
            Err(rubx::rux_debug::throw(rux_dbg_errs!(
                "Could not get the results from the join"
            )))
        }
    }

    fn wait(&self) -> Result<(), LizError> {
        loop {
            {
                let lock = self.results.read().map_err(|err| rux_dbg_erro!(err))?;
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
