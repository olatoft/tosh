use colored::*;
use std::env;
use std::fs::OpenOptions;
use std::io::{BufWriter, stdin, stdout, Write};
use std::path::{Path, /*PathBuf*/};
use std::process::{Child, Command, Stdio};


fn get_hostname() -> String {
    let session_manager: String = env::var("SESSION_MANAGER")
        .unwrap()
        .split(":")
        .take(1)
        .collect();

    let hostname_path: Vec<&str> = session_manager
        .split("/")
        .collect();

    let hostname = hostname_path
        .last()
        .unwrap()
        .to_string();

    hostname
}


fn print_prompt() {
    let user = env::var("USER").unwrap_or_default();
    let hostname = get_hostname();
    let home_dir = env::var("HOME").unwrap_or(String::from("/"));
    let mut current_dir = env::current_dir().unwrap();

    if current_dir.starts_with(&home_dir) {
        current_dir = Path::new("~/")
            .join(current_dir
                .strip_prefix(home_dir)
                .unwrap()
                .to_owned()
            );
    }

    // Print prompt
    print!("{}{}{}:{}$ ",
        user.green().bold(),
        "@".green().bold(),
        hostname.green().bold(),
        current_dir.to_str().unwrap().blue().bold());

    // Need to explicitly flush to ensure that the prompt is printed before
    // read_line is executed
    stdout().flush().unwrap();
}


fn main() {
    loop {
        print_prompt();
        let home_dir = env::var("HOME").unwrap_or(String::from("/"));

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let history = OpenOptions::new()
            .append(true)
            .create(true)
            .open("history.txt")
            .expect("Error: Could not open file history.txt");

        let mut history = BufWriter::new(history);

        history.write_all(input.as_bytes())
            .expect("Error: Could not write data to history.txt");

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    // Default directory: '/'
                    let new_dir = args
                        .peekable()
                        .peek()
                        .map_or(home_dir.as_str(), |x| *x);

                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                },

                "exit" => return,

                _ => {
                    let stdin = previous_command
                        .map_or(Stdio::inherit(),
                        |output: Child| Stdio::from(output.stdout.unwrap())
                    );

                    let stdout =
                        if commands.peek().is_some() {
                            // There is another command piped behind this one.
                            // Prepare to send output to the next command.
                            Stdio::piped()

                        } else {
                            // There are no more commands piped behind this one.
                            // Send output to shell stdout.
                            Stdio::inherit()
                        };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    // Don't accept another command until this one completes
                    // child.wait().unwrap();
                    match output {
                        Ok(output) => { previous_command = Some(output); },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        },
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            // Block until the final command has finished
            final_command.wait().unwrap();
        }
    }
}
