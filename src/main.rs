#![allow(unused_variables)]
#![allow(warnings)]
use std::env;
use std::fs::{File, read_to_string};
use std::io::{self, BufRead, BufReader};
use std::path::Path;
mod utils;
use utils::scanner::Scanner;
use utils::token::Token;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            eprintln!("Logs from your program will appear here!");


            let lines = read_lines(filename);
            eprintln!("lines are {:?}", lines);
            let last_line = lines.len() - 1;
            for (idx, line) in lines.iter().enumerate() {
                run(&line, idx, last_line);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn read_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    return Ok(reader.lines());
}

fn run(source: &str, current_line: usize, last_line: usize) {
    //eprintln!("the source is {source}");
    let mut scanner: Scanner = Scanner::new(source.to_string());
    let tokens: Vec<Token> = scanner.scan_tokens();
    //println!("total tokens {}", tokens.len());
    for token in tokens {
        println!("{}", token.to_string());
    }
    if source.is_empty() || current_line == last_line {
        scanner.exit();

    }
}
