use std::io::{self, Write};
use anyhow::{Context};
struct Input {
    module: String,
    command: Option<String>,
    argument: Option<String>,
}

enum Terminate {
    Error(String),
    Exit
}

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let _ = io::stdin()
            .read_line(&mut input)
            .with_context(|| format!("Failed reading input: {}", input));
        
        let args = match parse_commands(input) {
            Ok(x) => x,
            Err(Terminate::Exit) => break,
            Err(Terminate::Error(x)) => {
                eprintln!("{}", x);
                continue;
            }
        };

        println!("module: {}", args.module);
        println!("command: {}", args.command.unwrap_or("None given".to_string()));
        println!("argument: {}", args.argument.unwrap_or("None given".to_string()));
    }
}

fn parse_commands(input: String) -> Result<Input, Terminate> {
    let trimmed = input.trim();
    if trimmed.is_empty() { 
        return Err(Terminate::Error("Field cannot be blank!".into()));
    };

    let lowered = trimmed.to_lowercase(); // max 2 parts
    let mut parts = lowered.splitn(2, ' ');

    let cmd_part = match parts.next() {
        Some("exit") => return Err(Terminate::Exit),
        Some(x) => x,
        None => return Err(Terminate::Error("Field cannot be blank!".into())),
    };

    let argument = parts.next().map(String::from);

    let mut cmd_split = cmd_part.split(".");
    let module = cmd_split.next().unwrap().to_string();

    let command = cmd_split.next().map(String::from);

    Ok(Input { module, command, argument })
}
