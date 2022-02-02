use rlua::{Context, Table};

use crate::liz_execs;
use crate::utils;

use crate::liz_execs::Spawned;
use crate::LizError;

pub fn inject_execs<'a>(ctx: Context<'a>, liz: &Table<'a>) -> Result<(), LizError> {
    let run = ctx.create_function(|ctx, (path, args): (String, Option<Vec<String>>)| {
        utils::treat_error(ctx, crate::run(path, args))
    })?;

    let race = ctx
        .create_function(|ctx, path: String| utils::treat_error(ctx, crate::race_in(path, ctx)))?;

    let spawn = ctx.create_function(|ctx, (path, args): (String, Option<Vec<String>>)| {
        utils::treat_error(ctx, liz_execs::spawn(path, args, ctx))
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

    liz.set("run", run)?;
    liz.set("race", race)?;
    liz.set("spawn", spawn)?;
    liz.set("join", join)?;
    liz.set("cmd", cmd)?;
    liz.set("pause", pause)?;

    Ok(())
}
