use liz::utils;
use liz::LizError;
use std::path::Path;

fn main() -> Result<(), LizError> {
    let mut to_race: Vec<Box<dyn AsRef<Path>>> = Vec::new();
    let mut to_rise_args: Option<Vec<String>> = None;
    let mut first_arg = true;
    let mut getting_args = false;
    let mut verbose = false;
    for arg in std::env::args() {
        if !getting_args {
            if arg == "-h" || arg == "--help" {
                print_help();
                return Ok(());
            } else if arg == "-V" || arg == "--version" {
                println!("Liz (LuaWizard) {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            } else if arg == "-v" || arg == "--verbose" {
                verbose = true;
            } else if arg == "--" {
                getting_args = true;
            } else if arg.ends_with(".liz") || arg.ends_with(".lua") {
                to_race.push(Box::new(arg));
            } else {
                if !first_arg {
                    to_race.push(Box::new(format!("{}.liz", arg)));
                }
            }
        } else {
            if let Some(ref mut to_rise_args) = to_rise_args {
                to_rise_args.push(arg);
            } else {
                to_rise_args = Some(vec![arg]);
            }
        }
        if first_arg {
            first_arg = false;
        }
    }
    if to_race.is_empty() {
        to_race.push(Box::new("./default.liz"));
    }
    let first = &to_race[0];
    if verbose {
        if let Some(ref to_rise_args) = to_rise_args {
            println!("Rising with args: {:?}", to_rise_args);
        } else {
            println!("Rising with no args");
        }
    }
    let handler = liz::rise(first.as_ref(), to_rise_args)?;
    for race_path in to_race {
        let results = liz::race(race_path.as_ref(), &handler)?;
        if verbose {
            let display = utils::display(race_path.as_ref())?;
            println!("Raced the {} got: {:?}", display, results);
        }
    }
    Ok(())
}

fn print_help() {
    println!(
        "liz {}
Éverton M. Vieira <everton.muvi@gmail.com>

Liz ( LuaWizard ) is a library and a command that features a bunch of functionalities for lua scripts inside the liz global variable.
    
USAGE:
    liz [FLAGS] [PATH]... [-- ARGS] 

FLAGS:
    -v, --verbose   Prints the verbose information;
    -V, --version   Prints the version information;
    -h, --help      Prints the usage information;

PATH:
    Address of the scripts to be loaded and executed. It is not necessary to put the extension .liz but if no path was specified, Liz will try to execute the ./default.liz path.

ARGS:
    Arguments that can be passed for the scripts on the liz.args global variable.",
        env!("CARGO_PKG_VERSION")
    );
}
