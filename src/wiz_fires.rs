use rlua::{Context, Table};

use crate::liz_fires::{self, Spawned};
use crate::utils;
use crate::LizError;

pub fn inject_execs<'a>(lane: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let run = lane.create_function(|_, (path, args): (String, Option<Vec<String>>)| {
        utils::treat_error(crate::run(&path, &args))
    })?;

    let race =
        lane.create_function(|lane, path: String| utils::treat_error(crate::race_in(lane, &path)))?;

    let eval = lane.create_function(|lane, source: String| {
        utils::treat_error(crate::eval_in(lane, &source))
    })?;

    let spawn = lane.create_function(|lane, (path, args): (String, Option<Vec<String>>)| {
        utils::treat_error(liz_fires::spawn(lane, &path, &args))
    })?;

    let join =
        lane.create_function(|_, spawned: Spawned| utils::treat_error(liz_fires::join(spawned)))?;

    let join_all = lane.create_function(|_, spawneds: Vec<Spawned>| {
        utils::treat_error(liz_fires::join_all(spawneds))
    })?;

    let wait = lane.create_function(|_, spawned: Spawned| Ok(liz_fires::wait(spawned)))?;

    let wait_all =
        lane.create_function(|_, spawneds: Vec<Spawned>| Ok(liz_fires::wait_all(spawneds)))?;

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

    let pause = lane.create_function(|_, ()| Ok(liz_fires::pause()))?;

    let exe_ext = lane.create_function(|_, ()| Ok(liz_fires::exe_ext()))?;

    let get_os = lane.create_function(|_, ()| Ok(liz_fires::get_os()))?;

    let is_lin = lane.create_function(|_, ()| Ok(liz_fires::is_lin()))?;

    let is_mac = lane.create_function(|_, ()| Ok(liz_fires::is_mac()))?;

    let is_win = lane.create_function(|_, ()| Ok(liz_fires::is_win()))?;

    liz.set("run", run)?;
    liz.set("race", race)?;
    liz.set("eval", eval)?;
    liz.set("spawn", spawn)?;
    liz.set("join", join)?;
    liz.set("join_all", join_all)?;
    liz.set("wait", wait)?;
    liz.set("wait_all", wait_all)?;
    liz.set("cmd", cmd)?;
    liz.set("sleep", sleep)?;
    liz.set("pause", pause)?;
    liz.set("exe_ext", exe_ext)?;
    liz.set("get_os", get_os)?;
    liz.set("is_lin", is_lin)?;
    liz.set("is_mac", is_mac)?;
    liz.set("is_win", is_win)?;

    Ok(())
}
