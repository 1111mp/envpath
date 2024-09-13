use std::env;
use std::process;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = envpath::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("envpath error: {err}");
        process::exit(1);
    });

    if let Err(err) = envpath::run(config) {
        eprintln!("envpath error: {err}");
        process::exit(1);
    }
}
