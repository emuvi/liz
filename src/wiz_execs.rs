use rlua::{Context, Table};

use crate::execs;
use crate::utils;

use crate::execs::Spawned;
use crate::LizError;

pub fn inject_execs<'a>(ctx: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let run = ctx.create_function(|ctx, (path, args): (String, Option<Vec<String>>)| {
        utils::treat_error(ctx, crate::run(path, args))
    })?;

    let spawn = ctx.create_function(|_, (path, args): (String, Option<Vec<String>>)| {
        Ok(execs::spawn(path, args))
    })?;

    let join =
        ctx.create_function(|ctx, spawned: Spawned| utils::treat_error(ctx, execs::join(spawned)))?;

    let cmd = ctx.create_function(
        |ctx, (name, args, dir, print, throw): (String, Vec<String>, String, bool, bool)| {
            utils::treat_error(ctx, execs::cmd(&name, args.as_slice(), &dir, print, throw))
        },
    )?;

    let pause = ctx.create_function(|_, ()| Ok(execs::pause()))?;

    liz.set("run", run)?;
    liz.set("spawn", spawn)?;
    liz.set("join", join)?;
    liz.set("cmd", cmd)?;
    liz.set("pause", pause)?;

    Ok(())
}