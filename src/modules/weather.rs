use std::env;
use local_ip_address::local_ip;
use geolocation;
use serde::Deserialize;
pub struct Weather {
    pub city: Option<String>,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    current: WeatherDetails,
}

#[derive(Deserialize, Debug)]
struct WeatherDetails {
    temp: f64,
    weather: Vec<WeatherDescription>,
}

#[derive(Deserialize, Debug)]
struct WeatherDescription {
    description: String,
}

impl Weather {
    pub fn new(city: Option<String>) -> Self {
        Self { city }
    }

    pub async fn now(&self) -> Result<(), Box<dyn std::error::Error>> {
        let api_key = env::var("OPENWEATHER_API_KEY")
            .map_err(|_| "OPENWEATHER_API_KEY has not been set!")?;

        let (lat, lon) = match &self.city {
            Some(city) => {
                println!("Weather in {} now...", city);
                return Err("City weather not implemented yet.".into());
            },
            None => {
                let ip = local_ip().map_err(|_| "Could not get local IP")?;
                let loc = geolocation::find(ip.to_string().as_ref()).map_err(|_| "Could not geolocate IP")?;
                (loc.latitude, loc.longitude)
            },
        };

        let api_url = format!(
            "https://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&exclude=minutely,hourly,daily,alerts&appid={}",
            lat, lon, api_key
        );

        let response = reqwest::get(api_url)
            .await?
            .json::<CurrentWeather>()
            .await?;

        println!(
            "Current temperature: {}°C - {}",
            response.current.temp,
            response.current.weather.first().map_or("N/A", |w| w.description.as_str())
        );

        Ok(())
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