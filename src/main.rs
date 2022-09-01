use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    // This line creates an Editor with the default configuration options.
    let mut repl = Editor::<()>::new();

    // This if statement loads a file with the history of commands
    // If the file does not exists, it creates one.
    if repl.load_history("history").is_err() {
        println!("No previous history.");
    }

    // This is our infinite loop. We will be here until the user terminates the program.
    loop {
        // This line asks the user to input a command. You can add whatever you want in here as a prefix.
        let readline = repl.readline(">> ");

        // The readline method returns an Result. Which we now use a match statement to filter the result.
        match readline {
            Ok(line) => {
                repl.add_history_entry(line.as_str());
                println!("Line: {}", line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }

    // Here we are saving the commands into the file. Until now they are stored in memory.
    repl.save_history("history").unwrap();

}
