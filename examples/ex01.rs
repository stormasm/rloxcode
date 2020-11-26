// stand alone rustyline example

use rustyline::error::ReadlineError;
use rustyline::Editor;

#[derive(Debug)]
pub enum LineResult {
    Success(String),
    Error(String),
    Break,
    CtrlC,
    CtrlD,
    ClearHistory,
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

fn main() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
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
