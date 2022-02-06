use rlua::{Context, Table};

use crate::liz_execs;
use crate::utils;

use crate::liz_execs::Spawned;
use crate::LizError;

pub fn inject_execs<'a>(ctx: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let run = ctx.create_function(|ctx, (path, args): (String, Option<Vec<String>>)| {
        utils::treat_error(ctx, crate::run(&path, args))
    })?;

    let race = ctx
        .create_function(|ctx, path: String| utils::treat_error(ctx, crate::race_in(ctx, &path)))?;

    let spawn = ctx.create_function(|ctx, (path, args): (String, Option<Vec<String>>)| {
        utils::treat_error(ctx, liz_execs::spawn(ctx, &path, args))
    })?;

    let join = ctx.create_function(|ctx, spawned: Spawned| {
        utils::treat_error(ctx, liz_execs::join(spawned))
    })?;

    let cmd = ctx.create_function(
        |ctx,
         (name, args, dir, print, throw): (
            String,
            Vec<String>,
            Option<String>,
            Option<bool>,
            Option<bool>,
        )| {
            utils::treat_error(
                ctx,
                liz_execs::cmd(&name, args.as_slice(), dir, print, throw),
            )
        },
    )?;

    let pause = ctx.create_function(|_, ()| Ok(liz_execs::pause()))?;

    let get_os = ctx.create_function(|_, ()| Ok(liz_execs::get_os()))?;

    let is_lin = ctx.create_function(|_, ()| Ok(liz_execs::is_lin()))?;

    let is_mac = ctx.create_function(|_, ()| Ok(liz_execs::is_mac()))?;

    let is_win = ctx.create_function(|_, ()| Ok(liz_execs::is_win()))?;

    liz.set("run", run)?;
    liz.set("race", race)?;
    liz.set("spawn", spawn)?;
    liz.set("join", join)?;
    liz.set("cmd", cmd)?;
    liz.set("pause", pause)?;
    liz.set("get_os", get_os)?;
    liz.set("is_lin", is_lin)?;
    liz.set("is_mac", is_mac)?;
    liz.set("is_win", is_win)?;

    Ok(())
}
