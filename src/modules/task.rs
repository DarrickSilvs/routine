pub struct Task {
    pub command: Option<String>,
}

impl Task {
    pub fn new(command: Option<String>) -> Self {
        Self { command }
    }

    pub fn add(&self, argument: String) {
        println!("Adding {} to task list...", argument);
    }

    pub fn list(&self) {
        println!("Here's your task lists...");
    }

    pub fn help(&self) {
        println!("===== Command Lists =====");
        println!("- add: task.add <ARGS>");
        println!("- list: task.list");
        println!("=========================");
    }
}