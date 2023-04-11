use asciidoc2rs::Parser;

use std::{env, process};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match run_with_arguments(&args) {
            Ok(output) => {
                println!("{}", output);
                process::exit(0);
            }
            Err(error) => {
                eprintln!("error: {error}");
                process::exit(1);
            }
        }
    }

    match run_for_tck() {
        Ok(output) => {
            println!("{:?}", output);
            process::exit(0);
        }
        Err(error) => {
            eprintln!("error: {error}");
            process::exit(1);
        }
    }
}

fn run_with_arguments(args: &[String]) -> Result<String, Box<dyn Error>> {
    match read_file(args[1].clone()) {
        Ok(json) => {
            let parser = Parser::new(json.as_str());
            let doc = parser.parse_from_asg()?;
            Ok(format!("{:#?}", doc))
        },
        err => err
    }
}

fn run_for_tck() -> Result<String, Box<dyn Error>> {
    match read_stdin() {
        Ok(input) => {
            let parser = Parser::new(input.as_str());
            let doc = parser.parse_to_asg()?;
            Ok(format!("{:?}", serde_json::to_string(&doc)?))
        }
        err => err
    }
}

fn read_file(filename: String) -> Result<String, Box<dyn Error>> {
    let mut f = File::open(filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents)
}

fn read_stdin() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input)
}
