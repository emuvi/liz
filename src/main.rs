use liz::LizError;
use liz::{liz_dbg_bleb, liz_dbg_erro, liz_dbg_info};

fn main() -> Result<(), LizError> {
    let mut race_paths: Vec<String> = Vec::new();
    let mut rise_args: Option<Vec<String>> = None;
    let mut first_arg = true;
    let mut script_args = false;
    for arg in std::env::args() {
        if !script_args {
            if arg == "-h" || arg == "--help" {
                print_help();
                return Ok(());
            } else if arg == "-V" || arg == "--version" {
                println!("Liz (LuaWizard) {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            } else if arg == "-v" || arg == "--verbose" {
                liz::liz_debug::set_verbose(true);
            } else if arg == "-a" || arg == "--archive" {
                liz::liz_debug::set_archive(true);
            } else if arg == "-lu" || arg == "--lizs-update" {
                liz::liz_codes::set_lizs_update(true);
            } else if arg == "-dc" || arg == "--debug-calls" {
                liz::liz_debug::put_dbg_calls();
            } else if arg == "-dr" || arg == "--debug-reavs" {
                liz::liz_debug::put_dbg_reavs();
            } else if arg == "-ds" || arg == "--debug-steps" {
                liz::liz_debug::put_dbg_steps();
            } else if arg == "-dt" || arg == "--debug-tells" {
                liz::liz_debug::put_dbg_tells();
            } else if arg == "--" {
                script_args = true;
            } else if arg.ends_with(".liz") || arg.ends_with(".lua") {
                race_paths.push(arg);
            } else if !first_arg && !arg.starts_with("-") {
                race_paths.push(arg);
            } else if !first_arg {
                return Err(liz_dbg_erro!("Could not understand an argument", arg));
            }
        } else {
            if let Some(ref mut to_rise_args) = rise_args {
                to_rise_args.push(arg);
            } else {
                rise_args = Some(vec![arg]);
            }
        }
        if first_arg {
            first_arg = false;
        }
    }
    if race_paths.is_empty() {
        race_paths.push(format!("start"));
    }
    let first_path = &race_paths[0];
    let (rise_path, handler) =
        liz::rise(first_path, &rise_args).map_err(|err| liz_dbg_bleb!(err))?;
    race_paths[0] = rise_path;
    for race_path in race_paths {
        let results = liz::race(&race_path, &handler).map_err(|err| liz_dbg_bleb!(err))?;
        liz_dbg_info!("Race finished", race_path, results);
    }
    Ok(())
}

fn print_help() {
    println!(
        "liz {}
Éverton M. Vieira <everton.muvi@gmail.com>

Liz ( LuaWizard ) is a library and a command program that features a bunch of functionalities for lua scripts inside the liz global variable. 
    
USAGE:
    liz [FLAGS] [PATH]... [-- ARGS] 

FLAGS:
    -V, --version       Prints the version information.
    -v, --verbose       Prints the verbose information.
    -a, --archive       Saves the archive log on a file.
    -lu, --lizs-update  Updates the lizs scrips on each call.
    -dc, --debug-calls  If liz has debug symbols, debugs the functions calls.
    -dr, --debug-reavs  If liz has debug symbols, debugs the functions returns.
    -ds, --debug-steps  If liz has debug symbols, debugs the functions operations.
    -dt, --debug-tells  If liz has debug symbols, debugs the functions iterations.
    -h, --help          Prints this usage information.

PATH:
    Address of the script to be loaded and executed. It is not necessary to put the extension .liz but if no path was specified, Liz will try to execute the ./start.liz path.

ARGS:
    Arguments that can be passed for the scripts on the liz.args global variable.",
        env!("CARGO_PKG_VERSION")
    );
}
