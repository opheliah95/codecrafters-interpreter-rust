#![allow(unused_variables)]
#![allow(warnings)]   
use std::env;
use std::fs;
mod utils;
use utils::scanner::{Scanner};
use utils::token::{Token};
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            eprintln!("Logs from your program will appear here!");

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            // TODO: Uncomment the code below to pass the first stage
            if !file_contents.is_empty() {
                for line in file_contents.lines() {
                    run(line);
                }
               // println!("-> EOF  null");
            } else {
                println!("EOF  null"); // Placeholder, replace this line when implementing the scanner
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }

    
}


fn run (source: &str) {
    let mut scanner:Scanner = Scanner::new(source.to_string());
    let tokens: Vec<Token> = scanner.scan_tokens();
    //println!("total tokens {}", tokens.len());
    for token in tokens {
        println!("{}", token.to_string());
    }

    if scanner.has_error

}