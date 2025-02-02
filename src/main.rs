
use std::env::args;
use std::process::exit;
use std::fs;
use std::io;
use std::io::prelude::*;

mod scanner;
mod token;
mod generate_ast;
mod parser;
mod interpreter;
mod environment;

use scanner::Scanner;
use parser::Parser;
use interpreter::Interpreter;
use environment::Environment;

fn run_file(path: &str) -> Result<(), String>{
    let f = fs::read_to_string(path);
    let file = match f {
        Ok(file) => file,
        Err(err) => panic!("Problem opening the file: {err:?}")
    };

    run(file);

    Ok(())
}

fn run (source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens);
    println!("parser : {:?}\n", parser);

    // let expr = parser.expression();
    // println!("Expression: {:?}", expr);

    let parse = parser.parse();
    println!("Parse: {:?}\n", parse);

    let mut environment = Environment::new();
    let mut interpreter = Interpreter::new(environment);
    let _ = interpreter.interpret(parse);
    // println!("Eval: {:?}", eval.to_string());

}

fn run_prompt() {
    let stdin = io::stdin();
    let input = &mut String::new();

    println!("> ");
    loop {
        input.clear();
        let _ = match stdin.lock().read_line(input) {
            Ok(0) => {
                println!("\nDetected EOF (Ctrl+D). Exiting...");
                break;
            }
            Ok(value) => {
                if value == 1 {
                    break;
                };
            },
            Err(err) => panic!("Problem reading the input: {err:?}")
        };
        // println!("input: {:?}", input);
        run(input.to_string());
    }

    let input = input.replace("\n", " ");
    // println!("total input : {}", input);
}

fn main() {
    let args: Vec<String> = args().collect();
    println!("args: {:?}", args);
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        run_file(&args[0]); // Error aHNDLING
     } else {
        run_prompt();
     }
}
