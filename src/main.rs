use std::path::Path;
use liz::LizError;

fn main() -> Result<(), LizError> {
    let mut to_execute: Vec<Box<dyn AsRef<Path>>> = Vec::new();
    let mut to_execute_args: Option<Vec<String>> = None;
    let mut getting_args = false;
    for arg in std::env::args() {
        if !getting_args {
            if arg == "--" {
                getting_args = true;
            } else if arg.ends_with(".liz") || arg.ends_with(".lua") {
                to_execute.push(Box::new(arg));
            } else if arg == "-v" || arg == "--version" {
                let version = env!("CARGO_PKG_VERSION");
                println!("Liz (LuaWizard) {}", version);
                return Ok(());
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
        let default = Path::new("./default.liz");
        if default.exists() {
            to_execute.push(Box::new(default));
        }
    }
    if !to_execute.is_empty() {
        let handler = liz::start(to_execute_args)?;
        for path in to_execute {
            liz::load(path.as_ref(), &handler)?;
        }
    }
    Ok(())
}
