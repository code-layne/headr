use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: usize,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(reader) => {
                if config.bytes > 0 {
                    // read only bytes # chars from the buffer
                }
                for (line_num, line) in reader.lines().enumerate() {
                    if config.lines > line_num {
                        return Ok(())
                    }
                    let line = line?;
                    println!("{}", line);
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
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
            Arg::new("number-lines")
                .short('n')
                .long("number")
                .help("Number lines")
                .conflicts_with("number-chars"),
        )
        .arg(
            Arg::new("number-chars")
                .short('c')
                .long("number-chars")
                .help("Number chars"),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap_or_default()
            .map(|s| s.to_string())
            .collect(),
        lines: matches
            .get_one::<String>("number-lines")
            .map(|s| parse_positive_integer(s))
            .transpose()
            .map_err(|e| -> Box<dyn std::error::Error> {
                format!("illegal line count -- {e}").into()
            })?
            .unwrap_or(10),
        bytes: matches
            .get_one::<String>("number-chars")
            .map(|s| parse_positive_integer(s))
            .transpose()
            .map_err(|e| -> Box<dyn std::error::Error> {
                format!("illegal char count -- {e}").into()
            })?
            .unwrap_or(0),
    })
}
fn parse_positive_integer(val: &str) -> MyResult<usize> {
    match val.parse::<usize>() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(format!("{val} is not a valid positive integer").into()),
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[test]
fn test_parse_positive_integer() {
    assert_eq!(parse_positive_integer("3").unwrap(), 3);
    assert!(parse_positive_integer("0").is_err());
    assert!(parse_positive_integer("abc").is_err());
    assert!(parse_positive_integer("-1").is_err());
}