use crate::modules::{task::Task, *};
pub struct Input {
    pub module: Module,
    pub command: Option<String>,
    pub argument: Option<String>,
}

pub enum Terminate {
    Error(String),
    Exit,
}

pub fn parse_commands(input: String) -> Result<Input, Terminate> {
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
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let (command, argument) = (self.command.clone(), self.argument.clone());
        match self.module {
            Module::Spotify => {
                let spotify = Spotify::new(argument);

                match command.as_deref() {
                    Some("play") => spotify.play(),
                    Some("next") => spotify.next(),
                    Some("previous") => spotify.previous(),
                    None => spotify.help(),
                    Some(other) => eprintln!("Invalid spotify command: {}", other),
                }
            },
            Module::Task => {
                let task = Task::new();
                match command.as_deref() {
                    Some("add") => task.add(argument),
                    Some("done") => task.done(argument),
                    Some("clear") => task.clear(),
                    Some("list") => task.list(),
                    None => task.help(),
                    Some(other) => eprintln!("Invalid task command: {}", other),
                }
            },
            Module::Weather => {
                let weather = Weather::new(argument );

                match command.as_deref() {
                    Some("now") => weather.now().await?,
                    Some("week") => weather.week(),
                    None => weather.help(),
                    Some(other) => eprintln!("Invalid weather command: {}", other)
                }
            }
        };
        Ok(())
    }
}