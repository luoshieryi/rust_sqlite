mod repl;

use clap::{App, crate_version};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use crate::repl::{get_config, REPLHelper};

fn main() -> rustyline::Result<()> {
    env_logger::init();

    // Basic implementation with no arguments of our CLI application
    // For now just showing help and working as a place holder for future
    // implementations
    let _matcher = App::new("Rust-SQLite")
        .version("copy")
        .author("By luoshieryi")
        .about("Light version of SQLite developed with Rust")
        .get_matches();

    // Starting Rustyline with a default configuration
    let config = get_config();

    // Getting a new Rustyline Helper
    let helper = REPLHelper::new();

    // Initializing Rustyline Editor with set config and setting helper
    let mut repl = Editor::with_config(config);
    repl.set_helper(Some(helper));

    // This method loads history file into memory
    // If it doesn't exist, creates one
    // TODO: Check history file size and if too big, clean it.
    if repl.load_history("history").is_err() {
        println!("No previous history.");
    }

    // Counter is set to improve user experience and show user how many commands he has ran.
    let mut count = 1;
    loop {
        if count == 1 {
            // Friendly intro message for user
            println!("{}{}{}{}{}",
                     format!("Rust-SQLite - {}\n", crate_version!()),
                     "Enter .exit to quit.\n",
                     "Enter .help for usage hints.\n",
                     "Connected to a transient in-memory database.\n",
                     "Use .open FILENAME to reopen on a persistent database.\n",
                     // TODO: Get info about application name and version dynamically.
            );
        }

        let p = format!("rust-sqlite | {} >> ", count);
        repl.helper_mut()
            .expect("No helper found")
            .colored_prompt = format!("\x1b[1;32m{}\x1b[0m", p);
        // Source for ANSI Color information: http://www.perpetualpc.net/6429_colors.html#color_list
        // http://bixense.com/clicolors/


        // This line asks the user to input a command. You can add whatever you want in here as a prefix.
        let readline = repl.readline(&p);

        // The readline method returns an Result. Which we now use a match statement to filter the result.
        match readline {
            Ok(command) => {
                repl.add_history_entry(command.as_str());
                println!("Command: {}", command);
                if command.eq(".exit") {
                    break;
                } else {
                    println!("Error: unknown command or invalid arguments: '{}'. Enter '.help'", &command);
                }
            }
            Err(ReadlineError::Interrupted) => {
                break
            }
            Err(ReadlineError::Eof) => {
                break
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }

        count += 1;
    }

    // Here we are saving the commands into the file. Until now they are stored in memory.
    repl.save_history("history").unwrap();

    Ok(())

}
