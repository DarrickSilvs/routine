use std::{fs, path::Path};
use serde::{Serialize, Deserialize};

const FILE_PATH: &str = "tasks.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Task;

impl Task {
    pub fn new() -> Self {
        Task
    }

    fn read_tasks() -> Vec<String> {
        if !Path::new(FILE_PATH).exists() {
            return Vec::new();
        }

        let data = fs::read_to_string(FILE_PATH).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_default()
    }

    fn write_tasks(tasks: &[String]) {
        let data = serde_json::to_string_pretty(tasks).expect("Serialization failed");
        fs::write(FILE_PATH, data).expect("Writing to file failed");
    }

    pub fn add(&self, description: Option<String>) {
        match description {
            Some(task) => {
                let mut tasks = Self::read_tasks();
                if tasks.contains(&task) {
                    println!("Task already exists: {}", task);
                    return;
                }

                println!("Adding {} to task list...", task);
                tasks.push(task);
                tasks.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
                Self::write_tasks(&tasks);
            },
            None => eprintln!("Task can't be empty!"),
        }
    }

    pub fn list(&self) {
        let mut tasks = Self::read_tasks();
        if tasks.is_empty() {
            println!("No tasks yet.");
            return;
        }

        tasks.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

        println!("===== Task lists =====");
        for (i, task) in tasks.iter().enumerate() {
            println!("{}. {}", i + 1, task);
        }
        println!("=========================");
    }

    pub fn done(&self, index_str: Option<String>) {
        // self.description corresponds to task number in list!
        match index_str {
            Some(s) => {
                match s.parse::<usize>() {
                    Ok(index) => {
                        let mut tasks = Self::read_tasks();
                        if index < 1 || index > tasks.len() {
                            eprintln!("Invalid task number!");
                            return;
                        }

                        tasks.remove(index - 1);
                        println!("Task {index} removed.");

                        Self::write_tasks(&tasks);
                    },
                    Err(_) => eprintln!("Failed parsing input as a number.")
                };
            },
            None => eprintln!("Task number not provided."),
        }
    }

    pub fn clear(&self) {
        let mut tasks = Self::read_tasks();
        tasks.clear();
        Self::write_tasks(&tasks);
        println!("Task list cleared!");
    }

    pub fn help(&self) {
        println!("===== Command Lists =====");
        println!("- add: task-add <TASK>");
        println!("- done: task-done <TASK_NUMBER>");
        println!("- clear: task-clear");
        println!("- list: task-list");
        println!("=========================");
    }
}