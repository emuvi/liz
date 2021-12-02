use std::path::Path;

fn main() {
    let mut to_execute: Vec<Box<dyn AsRef<Path>>> = Vec::new();
    let mut to_execute_args: Option<Vec<String>> = None;
    let mut start_execute_args = false;
    for arg in std::env::args() {
        if !start_execute_args {
            if arg == "--" {
                start_execute_args = true;
            } else if arg.ends_with(".liz") || arg.ends_with(".lua") {
                to_execute.push(Box::new(arg));
            } else if arg == "-v" || arg == "--version" {
                let version = env!("CARGO_PKG_VERSION");
                println!("Liz (LuaWizard) {}", version);
                return;
            }
        } else {
            if let Some(ref mut liz_args) = to_execute_args {
                liz_args.push(arg);
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
    for path in to_execute {
        execute(path.as_ref(), to_execute_args.clone());
    }
}

fn execute(path: impl AsRef<Path>, args: Option<Vec<String>>) {
    match liz::execute(path, args) {
        Ok(result) => println!("{}", result),
        Err(error) => eprintln!("{}", error),
    };
}
