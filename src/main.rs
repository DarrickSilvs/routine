use std::io::{self, Write};
use anyhow::{Context};
use input::*;
use dotenv::dotenv;
use tokio;

mod input;
mod modules;

#[tokio::main]
async fn main() {
    dotenv().ok();
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

       if let Err(e) = args.run().await {
            eprintln!("Command failed: {}", e);
        }
    }
}