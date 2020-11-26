pub mod classes;
pub mod environment;
pub mod error;
pub mod functions;
pub mod interpreter;
pub mod lexer;
pub mod object;
pub mod parser;
pub mod resolver;
pub mod statement;
pub mod token;

use crate::error::LoxError;
use crate::interpreter::Interpreter;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Write;

pub fn run_prompt_code(interpreter: &mut interpreter::Interpreter, buffer: &String) {
    let (tokens, lexer_errors) = lexer::lex(&buffer);
    print_errors(&lexer_errors);

    let (statements, parser_errors) = parser::parse(&tokens);
    print_errors(&parser_errors);

    if !lexer_errors.is_empty() || !parser_errors.is_empty() {
        std::process::exit(64);
    }

    let scopes = resolver::resolve(&statements);
    if scopes.is_err() {
        std::process::exit(64);
    }
    interpreter.add_scopes(scopes.unwrap());

    interpreter
        .interpret(statements)
        .expect("Interpreter error: ");
}

pub fn run_prompt() {
    let mut interpreter = Interpreter::new();
    loop {
        print!("> ");
        io::stdout().flush().expect("Could not write to stdout");
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => run_prompt_code(&mut interpreter, &mut buffer),
            Err(error) => eprintln!("error reading line: {}", error),
        }
    }
}

pub fn run_file_code(code: &str) {
    let mut interpreter = Interpreter::new();
    let (tokens, lexer_errors) = lexer::lex(&code);
    print_errors(&lexer_errors);

    let (statements, parser_errors) = parser::parse(&tokens);
    print_errors(&parser_errors);

    if !lexer_errors.is_empty() || !parser_errors.is_empty() {
        std::process::exit(64);
    }

    let scopes = resolver::resolve(&statements);
    if scopes.is_err() {
        std::process::exit(64);
    }
    interpreter.add_scopes(scopes.unwrap());

    interpreter
        .interpret(statements)
        .expect("Interpreter error: ");
}

pub fn run_file(filename: &str) {
    let mut file = File::open(filename).expect("Could not read file: ");
    let mut code = String::new();
    file.read_to_string(&mut code)
        .expect("Could not read file: ");
    run_file_code(&code);
}

pub fn print_errors(errors: &Vec<LoxError>) {
    for error in errors {
        eprintln!("{}", error);
    }
}
