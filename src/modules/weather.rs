pub struct Weather {
    pub city: Option<String>,
}

impl Weather {
    pub fn new(city: Option<String>) -> Self {
        Self { city }
    }

    pub fn now(&self) {
        match &self.city {
            Some(city) => println!("Weather in {} now...", city),
            None => println!("Weather currently..."),
        }
    }

    pub fn week(&self) {
        match &self.city {
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