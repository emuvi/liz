use liz::LizError;
use std::path::Path;

fn main() -> Result<(), LizError> {
    let mut to_execute: Vec<Box<dyn AsRef<Path>>> = Vec::new();
    let mut to_execute_args: Option<Vec<String>> = None;
    let mut getting_args = false;
    for arg in std::env::args() {
        if !getting_args {
            if arg == "--" {
                getting_args = true;
            } else if arg == "-v" || arg == "--version" {
                let version = env!("CARGO_PKG_VERSION");
                println!("Liz (LuaWizard) {}", version);
                return Ok(());
            } else if arg == "-h" || arg == "--help" {
                print_help();
                return Ok(());
            } else if arg.ends_with(".liz") || arg.ends_with(".lua") {
                to_execute.push(Box::new(arg));
            } else {
                to_execute.push(Box::new(format!("{}.liz", arg)));
            }
        } else {
            if let Some(ref mut to_execute_args) = to_execute_args {
                to_execute_args.push(arg);
            } else {
                to_execute_args = Some(vec![arg]);
            }
        }
    }
    if to_execute.is_empty() {
        to_execute.push(Box::new("./default.liz"));
    }
    let handler = liz::start(to_execute_args)?;
    for path in to_execute {
        liz::load(path.as_ref(), &handler)?;
    }
    Ok(())
}

fn print_help() {
    println!(
        "liz {}
Éverton M. Vieira <everton.muvi@gmail.com>
LuaWizard - Features a bunch of functionalities for lua scripts inside the liz global variable.
    
USAGE:
    liz [FLAGS] [PATH]... [-- ARGS] 

FLAGS:
    -v, --version   Prints the version information;
    -h, --help      Prints the help information;

PATH:
    Address of the scripts to be loaded and executed. It is not necessary to put the extension .liz and if no path was specified we wil try to execute the ./default.liz path.

ARGS:
    Arguments that can be passed for the scripts on the liz.args global variable.",
        env!("CARGO_PKG_VERSION")
    );
}
