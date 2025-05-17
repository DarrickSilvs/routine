pub struct Weather {
    pub command: Option<String>,
    pub location: Option<String>,
}

impl Weather {
    pub fn new(command: Option<String>, location: Option<String>) -> Self {
        Self { command, location }
    }

    pub fn now(&self) {
        match &self.location {
            Some(city) => println!("Weather in {} now...", city),
            None => println!("Weather currently..."),
        }
    }

    pub fn week(&self) {
        match &self.location {
            Some(city) => println!("Weather in {} this week...", city),
            None => println!("Weather this week..."),
        }
    }

    pub fn help(&self) {
        println!("===== Command Lists =====");
        println!("- now: weather.now [LOCATION]");
        println!("- week: weather.week [LOCATION]");
        println!("=========================");
    }
}