pub struct Task {
    pub description: Option<String>,
    pub tasks: Vec<String>,
}

impl Task {
    pub fn new() -> Self {
        Self { description: None, tasks: Vec::new() }
    }
    pub fn add(&mut self) {
        match &self.description {
            Some(task) => {
                println!("Adding {} to task list...", task);
                self.tasks.push(task.clone());
            },
            None => eprintln!("Task can't be empty!"),
        }
    }

    pub fn list(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks yet.");
            return;
        }

        println!("===== Task lists =====");
        for (i, task) in self.tasks.iter().enumerate() {
            println!("{}. {}", i + 1, task);
        }
        println!("=========================");
    }

    pub fn done(&mut self) {
        // self.description corresponds to task number in list!
        match &self.description {
            Some(index_str) => {
                match index_str.parse::<usize>() {
                    Ok(index) if index == 0 || index > self.tasks.len() => eprintln!("Invalid task number!"),
                    Ok(index) => {
                        self.tasks.swap_remove(index - 1);
                        println!("Task {index} removed.");
                    },
                    Err(_) => eprintln!("Failed parsing input as a number.")
                };
            },
            None => eprintln!("Task can't be empty!"),
        }
    }

    pub fn help(&self) {
        println!("===== Command Lists =====");
        println!("- add: task.add <TASK>");
        println!("- done: task.done <TASK_NUMBER>");
        println!("- list: task.list");
        println!("=========================");
    }
}