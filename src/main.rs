use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

const GREEN: &str = "\x1b[32m";
/*
const RED: &str = "\x1b[31m";
const ANSI_COLOR_YELLOW: &str = "\x1b[33m";
const ANSI_COLOR_BLUE: &str = "\x1b[34m";
const ANSI_COLOR_MAGENTA: &str = "\x1b[35m";
const ANSI_COLOR_CYAN: &str = "\x1b[36m";
const ANSI_BOLD: &str = "\x1b[1m";
*/

const RESET: &str = "\x1b[0m";

fn main() {
    loop {
        let current_dir = env::current_dir().unwrap();

        print!("{}{} > {}",GREEN, current_dir.display(), RESET);
        if let Err(e) = stdout().flush() {
            eprintln!("Error encountered : {}", e);
        }

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    previous_command = None;
                }
                "exit" => return,
                command => {
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => {
                            previous_command = Some(output);
                        }
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            if let Err(e) = final_command.wait() {
                eprintln!("Error encountered : {}", e);
            }
        }
    }
}
