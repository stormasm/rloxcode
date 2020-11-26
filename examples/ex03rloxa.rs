// The whole idea of this example is to be able
// to call rlox methods by way of the library.

use rlox::{run_file, run_prompt};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        run_prompt();
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        eprintln!("Unexpected number of arguments. Expected none (interactive) or one(file).");
    }
}
