use rlua::{Context, Table};

use crate::liz_codes;
use crate::liz_debug::{dbg_bleb, dbg_call, dbg_reav, dbg_step};
use crate::liz_fires::{self, Spawned};
use crate::liz_paths;
use crate::utils;
use crate::LizError;

fn lane_suit_path<'a>(lane: Context<'a>, path: String) -> Result<String, LizError> {
    dbg_call!(path);
    let suit_path = liz_codes::liz_suit_path(&path).map_err(|err| dbg_bleb!(err))?;
    dbg_step!(suit_path);
    let suit_path = if liz_paths::is_relative(&suit_path) {
        let stack_dir = utils::get_stacked_dir(lane).map_err(|err| dbg_bleb!(err))?;
        liz_paths::path_join(&stack_dir, &suit_path).map_err(|err| dbg_bleb!(err))?
    } else {
        suit_path
    };
    dbg_reav!(Ok(suit_path));
}

pub fn inject_execs<'a>(lane: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let run = lane.create_function(|lane, (path, args): (String, Option<Vec<String>>)| {
        let lane_path = match utils::treat_error(lane_suit_path(lane, path)) {
            Ok(lane_path) => lane_path,
            Err(err) => return Err(err),
        };
        dbg_step!(lane_path);
        utils::treat_error(crate::run(&lane_path, &args))
    })?;

    let eval = lane
        .create_function(|lane, source: String| utils::treat_error(crate::eval_in(lane, source)))?;

    let race =
        lane.create_function(|lane, path: String| utils::treat_error(crate::race_in(lane, &path)))?;

    let run_wd = lane.create_function(|_, (relative_path, args): (String, Option<Vec<String>>)| {
        utils::treat_error(liz_fires::run_wd(&relative_path, &args))
    })?;

    let race_wd = lane.create_function(|lane, relative_path: String| {
        utils::treat_error(liz_fires::race_wd(lane, &relative_path))
    })?;

    let spawn = lane.create_function(|lane, (path, args): (String, Option<Vec<String>>)| {
        utils::treat_error(liz_fires::spawn(lane, &path, &args))
    })?;

    let join =
        lane.create_function(|_, spawned: Spawned| utils::treat_error(liz_fires::join(spawned)))?;

    let join_all = lane.create_function(|_, spawneds: Vec<Spawned>| {
        utils::treat_error(liz_fires::join_all(spawneds))
    })?;

    let wait =
        lane.create_function(|_, spawned: Spawned| utils::treat_error(liz_fires::wait(spawned)))?;

    let wait_all = lane.create_function(|_, spawneds: Vec<Spawned>| {
        utils::treat_error(liz_fires::wait_all(spawneds))
    })?;

    let cmd = lane.create_function(
        |_,
         (name, args, dir, print, throw): (
            String,
            Vec<String>,
            Option<String>,
            Option<bool>,
            Option<bool>,
        )| {
            utils::treat_error(liz_fires::cmd(&name, args.as_slice(), dir, print, throw))
        },
    )?;

    let sleep = lane.create_function(|_, millis: u64| Ok(liz_fires::sleep(millis)))?;

    let pause = lane.create_function(|_, ()| utils::treat_error(liz_fires::pause()))?;

    let exe_ext = lane.create_function(|_, ()| Ok(liz_fires::exe_ext()))?;

    let liz_dir = lane.create_function(|_, ()| utils::treat_error(liz_fires::liz_dir()))?;

    let liz_exe = lane.create_function(|_, ()| utils::treat_error(liz_fires::liz_exe()))?;

    let get_os = lane.create_function(|_, ()| Ok(liz_fires::get_os()))?;

    let is_lin = lane.create_function(|_, ()| Ok(liz_fires::is_lin()))?;

    let is_mac = lane.create_function(|_, ()| Ok(liz_fires::is_mac()))?;

    let is_win = lane.create_function(|_, ()| Ok(liz_fires::is_win()))?;

    liz.set("run", run)?;
    liz.set("eval", eval)?;
    liz.set("race", race)?;
    liz.set("run_wd", run_wd)?;
    liz.set("race_wd", race_wd)?;
    liz.set("spawn", spawn)?;
    liz.set("join", join)?;
    liz.set("join_all", join_all)?;
    liz.set("wait", wait)?;
    liz.set("wait_all", wait_all)?;
    liz.set("cmd", cmd)?;
    liz.set("sleep", sleep)?;
    liz.set("pause", pause)?;
    liz.set("liz_dir", liz_dir)?;
    liz.set("liz_exe", liz_exe)?;
    liz.set("exe_ext", exe_ext)?;
    liz.set("get_os", get_os)?;
    liz.set("is_lin", is_lin)?;
    liz.set("is_mac", is_mac)?;
    liz.set("is_win", is_win)?;

    Ok(())
}
