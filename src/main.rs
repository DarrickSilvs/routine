use std::io::{self, Write};
use anyhow::{Context};
struct Input {
    module: Module,
    command: Option<String>,
    argument: Option<String>,
}

enum Terminate {
    Error(String),
    Exit
}

enum Module {
    Weather,
    Spotify,
    Task,
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

        args.run();
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

    let module = cmd_split.next().unwrap();
    let module = match module {
        "spotify" => Module::Spotify,
        "weather" => Module::Weather,
        "task" => Module::Task,
        _ => return Err(Terminate::Error("Invalid module!".into())),
    };

    let command = cmd_split.next().map(String::from);

    Ok(Input { module, command, argument })
}

impl Input {
    fn run(&self) {
        let (command, argument) = (self.command.clone(), self.argument.clone());
        match self.module {
            Module::Spotify => {
                let spotify = Spotify::new(command);

                match spotify.command.as_deref() {
                    Some("play") => spotify.play(argument.unwrap()),
                    Some("next") => spotify.next(),
                    Some("previous") => spotify.previous(),
                    None => spotify.help(),
                    Some(other) => eprintln!("Invalid spotify command: {}", other),
                }
            },
            Module::Task => {
                let task = Task::new(command);

                match task.command.as_deref() {
                    Some("add") => task.add(argument.unwrap()),
                    Some("list") => task.list(),
                    None => task.help(),
                    Some(other) => eprintln!("Invalid task command: {}", other),
                }
            },
            Module::Weather => {
                let weather = Weather::new(command, argument );

                match weather.command.as_deref() {
                    Some("now") => weather.now(),
                    Some("week") => weather.week(),
                    None => weather.help(),
                    Some(other) => eprintln!("Invalid weather command: {}", other)
                }
            }
        };
    }
}

struct Spotify {
    command: Option<String>,
}

impl Spotify {
    fn new(command: Option<String>) -> Self {
        Self { command }
    }

    fn play(&self, song: String) {
        println!("Playing {} on spotify...", song);
    }

    fn next(&self) {
        println!("Skipping to next song...");
    }

    fn previous(&self) {
        println!("Going back to previous song...");
    }

    fn help(&self) {
        println!("===== Command Lists =====");
        println!("- play: spotify.play [song]");
        println!("- next: spotify.next");
        println!("- previous: spotify.previous");
        println!("=========================");
    }
}

struct Weather {
    command: Option<String>,
    location: Option<String>,
}

impl Weather {
    fn new(command: Option<String>, location: Option<String>) -> Self {
        Self { command, location }
    }

    fn now(&self) {
        match &self.location {
            Some(city) => println!("Weather in {} now...", city),
            None => println!("Weather currently..."),
        }
    }

    fn week(&self) {
        match &self.location {
            Some(city) => println!("Weather in {} this week...", city),
            None => println!("Weather this week..."),
        }
    }

    fn help(&self) {
        todo!()
    }
}

struct Task {
    command: Option<String>,
}

impl Task {
    fn new(command: Option<String>) -> Self {
        Self { command }
    }

    fn add(&self, argument: String) {
        println!("Adding {} to task list...", argument);
    }

    fn list(&self) {
        println!("Here's your task lists...");
    }

    fn help(&self) {
        println!("===== Command Lists =====");
        println!("- add: task.add [args]");
        println!("- list: task.list");
        println!("=========================");
    }
}