use liz::LizError;

fn main() -> Result<(), LizError> {
    for arg in std::env::args() {
        if arg == "-v" || arg == "--version" {
            let version = env!("CARGO_PKG_VERSION");
            println!("Liz {}", version);
        } else if arg.ends_with(".lua") {
            match liz::execute(arg) {
                Ok(result) => println!("{}", result),
                Err(error) => eprintln!("{}", error),
            };
        }
    }
    Ok(())
}
