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
    line: &str,
) -> LineResult {
    if line.trim() == "" {
        LineResult::Success(line.to_string())
    } else {
        let line = chomp_newline(line);
        LineResult::Success(line.to_string())
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

                // process_line(&line);


                let _crline = match convert_rustyline_result_to_string(Ok(line)) {
                    LineResult::Success(s) => process_line(&s),
                    x => x,
                };



                // println!("Line: {}", line);
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
