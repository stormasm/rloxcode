use rustyline::error::ReadlineError;
use rustyline::Editor;

use rlox::interpreter::Interpreter;
// use rlox::{run_prompt_code};

use rlox::{lexer, parser, resolver, print_errors};




#[derive(Debug)]
pub enum LineResult {
    Success(String),
    Error(String),
    Break,
    CtrlC,
    CtrlD,
    ClearHistory,
}

fn chomp_newline(s: &str) -> &str {
    if let Some(s) = s.strip_suffix('\n') {
        s
    } else {
        s
    }
}

fn convert_rustyline_result_to_string(input: Result<String, ReadlineError>) -> LineResult {
    match input {
        Ok(s) if s == "history -c" || s == "history --clear" => LineResult::ClearHistory,
        Ok(s) => LineResult::Success(s),
        Err(ReadlineError::Interrupted) => LineResult::CtrlC,
        Err(ReadlineError::Eof) => LineResult::CtrlD,
        Err(err) => {
            println!("Error: {:?}", err);
            LineResult::Break
        }
    }
}

fn process_line(
    interpreter: &mut rlox::interpreter::Interpreter,
    line: &str,
) -> LineResult {
    if line.trim() == "" {
        LineResult::Success(line.to_string())
    } else {
        let line = chomp_newline(line);
        let (tokens, lexer_errors) = lexer::lex(&line);
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

        LineResult::Success(line.to_string())
    }
}

fn main() {
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    let mut interpreter = Interpreter::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let _crline = match convert_rustyline_result_to_string(Ok(line)) {
                    LineResult::Success(s) => process_line(&mut interpreter,&s),
                    x => x,
                };
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}
