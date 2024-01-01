/* There is a main function that is the starting point.
From the main function, it can either do `run_file` or
`run_prompt` -- both of which are simply entrypoints into
the `run` function. */


use std::env::args;
use std::io::Write;
use std::path::PathBuf;
use std::fs::read_to_string;

fn main() {
    /* get the command line args.
    - If there is more than one (in addition to
      the first arg, which is the file path), then we
      want to break and tell the user we only want one arg.
    - if there is only one, then find that file and run the
      code inside it.
    - if there are no arguments, then run rustlox as an interpreter */
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        println!("Usage: rustlox [script]");
    } else if args.len() == 2 {
        let path: PathBuf = PathBuf::from(&args[1]);
        run_file(&path);
    } else {
        run_prompt();
    }
}


/// Call the interpreter to run the code inside the file.
fn run_file(path: &PathBuf) {
    println!("Running file: {:?}", path);

    let contents = read_to_string(path)
        .expect("Something went wrong reading the file");

    run(&contents);
}

/// Call the interpreter to run the code inside the prompt.
/// 
/// This is a REPL (Read-Eval-Print-Loop) that will keep
/// running until the user exits.
/// 
/// to do this, do a loop that will keep printing the prompt
/// and then reading the input from the user. Then, run the
/// code inside the interpreter.  
/// 
/// Break the loop if the line is empty or if the user
/// types `exit` or `quit`.
/// 
/// At the very beginning, tell the user that they can exit by
/// typing `exit` or `quit`.
fn run_prompt() {
    println!("Running. Type 'exit' or 'quit' to exit");

    loop {

        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" || input.trim() == "quit" {
            break;
        }

        run(&(input.trim()));
    }
}

/// Run the code inside the interpreter.
/// 
/// for now, just print the code.
fn run(code: &str) {
    println!("Running code: {}", code);
}

