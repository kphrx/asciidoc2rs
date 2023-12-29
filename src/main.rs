use serde::{Deserialize, Serialize};

use asciidoc2rs::Parser;

use std::error::Error;

use std::io;
use std::io::prelude::*;
use std::{env, process};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Stdin {
    Block(InputBody),
    Inline(InputBody),
}

#[derive(Serialize, Deserialize)]
pub struct InputBody {
    contents: String,
    path: String,
}

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
            println!("{}", output);
            process::exit(0);
        }
        Err(error) => {
            eprintln!("error: {error}");
            process::exit(1);
        }
    }
}

fn run_with_arguments(_args: &[String]) -> Result<String, Box<dyn Error>> {
    Err("not implemented".into())
}

fn run_for_tck() -> Result<String, Box<dyn Error>> {
    match read_stdin() {
        Ok(input) => {
            let (Stdin::Block(content) | Stdin::Inline(content)) =
                serde_json::from_str(input.as_str())?;
            let InputBody { contents: text, .. } = content;
            let parser = Parser::new(text.as_str());
            let doc = parser.parse()?;
            Ok((serde_json::to_string(&doc)?).to_string())
        }
        err => err,
    }
}

fn read_stdin() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    Ok(input)
}
