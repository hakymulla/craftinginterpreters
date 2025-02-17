
use std::env::args;
use std::process::exit;
use std::fs;
use std::io;
use std::io::prelude::*;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use std::rc::Rc;


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
    let mut interpreter = Interpreter::new();
    let file = match f {
        Ok(file) => file,
        Err(err) => panic!("Problem opening the file: {err:?}")
    };

    run(file, &mut interpreter);

    Ok(())
}

fn run (source: String, interpreter: &mut Interpreter) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens);
    // println!("parser : {:?}\n", parser);

    // let expr = parser.expression();
    // println!("Expression: {:?}", expr);

    let parse = parser.parse();
    // println!("Parse: {:?}\n", parse);

    // let environment = Rc::new(Environment::new());
    let value = interpreter.interpret(parse);
    println!("OutputVal: {:?}", value.to_string());

}

fn run_prompt() {
    let stdin = io::stdin();
    let input = &mut String::new();
    let mut interpreter = Interpreter::new();

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
        run(input.to_string(), &mut interpreter);
    }

    println!("INput in run pormpmt");

    let input = input.replace("\n", " ");
    println!("total input : {}", input);
}

fn main() {
    let args: Vec<String> = args().collect();
    println!("args: {:?}", args);
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        let _ = run_file(&args[0]); // Error aHNDLING
     } else {
        run_prompt();
     }
}

// {var b=20; print b;}