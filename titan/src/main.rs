use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod token;
mod lexer;

fn usage() {
    println!("usage: titan [help, docs, run] [file]");
    println!("Run | titan help | for more");
    std::process::exit(0x0000);
}

fn help() {
    println!("Titan Help");
    println!("==================");
    println!("run - If you supply a file it run that file.");
    println!("docs - Will guide you to the Titan docs.");
    println!("help - Explains the commands.");

    std::process::exit(0x0000);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage();
    }

    if args[1] == "help" {
        help();
    }

    if args[1] == "docs" {

    }

    if args[1] == "run" {
        let file = File::open(&args[2]).unwrap();
        let reader = BufReader::new(file);

        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap(); 
            let mut lexer = lexer::Lexer::new(line);
            lexer.lex();
        }
    }
}
