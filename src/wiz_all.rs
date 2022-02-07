use rlua::{Context, MultiValue, Value};

use crate::liz_files;
use crate::wiz_codes;
use crate::wiz_execs;
use crate::wiz_files;
use crate::wiz_texts;
use crate::wiz_trans;

use crate::utils::{self, debug};
use crate::LizError;

pub fn inject_all(ctx: Context, path: &str, args: Option<Vec<String>>) -> Result<(), LizError> {
    let liz = ctx.create_table()?;
    liz.set("args", args)
        .map_err(|err| debug!("set", &["args", args], err))?;

    let path = utils::add_liz_extension(path);
    let path = if liz_files::is_symlink(&path) {
        liz_files::path_walk(&path).map_err(|err| debug!("path_walk", &["path", path], err))?
    } else {
        path.into()
    };

    let rise_pwd = liz_files::pwd().map_err(|err| debug!("pwd", (), err))?;
    liz.set("rise_pwd", rise_pwd)
        .map_err(|err| debug!("set", &["rise_pwd", rise_pwd], err))?;

    let rise_dir =
        liz_files::path_parent(&path).map_err(|err| debug!("path_parent", &["path", path], err))?;
    utils::put_stack_dir(&ctx, &liz, rise_dir.clone())
        .map_err(|err| debug!("put_stack_dir", &["rise_dir", rise_dir], err))?;
    liz.set("rise_dir", rise_dir)
        .map_err(|err| debug!("set", &["rise_dir", rise_dir], err))?;

    let rise_path = liz_files::path_absolute(&path)
        .map_err(|err| debug!("path_absolute", &["path", path], err))?;
    liz.set("rise_path", rise_path)
        .map_err(|err| debug!("set", &["rise_path", rise_path], err))?;

    let print_stack_dir = ctx
        .create_function(|ctx, ()| utils::treat_error(ctx, utils::print_stack_dir(ctx)))
        .map_err(|err| debug!("create_function", "print_stack_dir", err))?;

    let last_stack_dir = ctx
        .create_function(|ctx, ()| utils::treat_error(ctx, utils::last_stack_dir(ctx)))
        .map_err(|err| debug!("create_function", "last_stack_dir", err))?;

    let to_json_multi = ctx
        .create_function(|ctx, values: MultiValue| {
            utils::treat_error(ctx, utils::to_json_multi(values))
        })
        .map_err(|err| debug!("create_function", "to_json_multi", err))?;

    let to_json = ctx
        .create_function(|ctx, value: Value| utils::treat_error(ctx, utils::to_json(value)))
        .map_err(|err| debug!("create_function", "to_json", err))?;

    let from_json = ctx
        .create_function(|ctx, source: String| {
            utils::treat_error(ctx, utils::from_json(ctx, source))
        })
        .map_err(|err| debug!("create_function", "from_json", err))?;

    liz.set("print_stack_dir", print_stack_dir)
        .map_err(|err| debug!("set", "print_stack_dir", err))?;
    liz.set("last_stack_dir", last_stack_dir)
        .map_err(|err| debug!("set", "last_stack_dir", err))?;
    liz.set("to_json_multi", to_json_multi)
        .map_err(|err| debug!("set", "to_json_multi", err))?;
    liz.set("to_json", to_json)
        .map_err(|err| debug!("set", "to_json", err))?;
    liz.set("from_json", from_json)
        .map_err(|err| debug!("set", "from_json", err))?;

    wiz_codes::inject_codes(ctx, &liz).map_err(|err| debug!("inject_codes", (), err))?;
    wiz_execs::inject_execs(ctx, &liz).map_err(|err| debug!("inject_execs", (), err))?;
    wiz_files::inject_files(ctx, &liz).map_err(|err| debug!("inject_files", (), err))?;
    wiz_texts::inject_texts(ctx, &liz).map_err(|err| debug!("inject_texts", (), err))?;
    wiz_trans::inject_trans(ctx, &liz).map_err(|err| debug!("inject_trans", (), err))?;

    let globals = ctx.globals();
    globals
        .set("liz", liz)
        .map_err(|err| debug!("set", "liz", err))?;

    Ok(())
}
