use clap::{Arg, ArgAction, Command};
use std::error::Error;

#[derive(Debug)]
pub struct Config {}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(_config: Config) -> MyResult<()> {
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let _matches = Command::new("headr")
        .version("0.1.0")
        .author("shepherd")
        .about("Rust head")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("Number lines")
                .action(ArgAction::SetTrue)
                .conflicts_with("number-nonblank"),
        )
        .arg(
            Arg::new("number-nonblank")
                .short('b')
                .long("number-nonblank")
                .help("Number non-blank lines")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    Ok(Config {})
}
