use std::env;
use geolocation;
use serde::Deserialize;
use std::net::IpAddr;
pub struct Weather {
    pub city: Option<String>,
}

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<WeatherDescription>,
    main: Main,
    name: String,
    sys: Sys
}

#[derive(Deserialize, Debug)]
struct WeatherDescription {
    description: String,
}

#[derive(Deserialize, Debug)]
struct Sys {
    country: String
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
}

#[derive(Deserialize, Debug)]
struct ForecastResponse {
    city: City,
    list: Vec<ForecastDay>
}

#[derive(Deserialize, Debug)]
struct City {
    name: String,
    country: String
}

#[derive(Deserialize, Debug)]
struct ForecastDay {
    temp: Temp,
    weather: Vec<WeatherDescription>,
}

#[derive(Deserialize, Debug)]
struct Temp {
    min: f64,
    max: f64
}

#[derive(Deserialize, Debug)]
struct Geocoding {
    lat: f64,
    lon: f64
}

impl Weather {
    pub fn new(city: Option<String>) -> Self {
        Self { city }
    }

    pub async fn now(&self) -> Result<(), Box<dyn std::error::Error>> {
        let api_key = env::var("OPENWEATHER_API_KEY")
            .map_err(|_| "OPENWEATHER_API_KEY has not been set!".to_string())?;

        let (lat, lon) = self.get_loc(&self.city).await?;

        let api_url = format!(
            "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",
            lat, lon, api_key
        );

        let resp = reqwest::get(api_url)
            .await?
            .json::<WeatherResponse>()
            .await?;

        let description = resp.weather
            .get(0)
            .map(|w| w.description.clone())
            .unwrap_or("N/A".to_string());

        let to_c = |k: f64| k - 273.15;
        let temp = to_c(resp.main.temp);
        let feels_like = to_c(resp.main.feels_like);
        let temp_min = to_c(resp.main.temp_min);
        let temp_max = to_c(resp.main.temp_max);

        let name = match &self.city {
            Some(city) => city,
            None => &resp.name,
        };

        let country = &resp.sys.country;

        println!("Current weather at {}, {}: {}", name, country, description);
        println!("Temp        : {:.1} °C", temp);
        println!("Feels like  : {:.1} °C", feels_like);
        println!("Temp min    : {:.1} °C", temp_min);
        println!("Temp max    : {:.1} °C", temp_max);

        Ok(())
    }

    pub async fn week(&self) -> Result<(), Box<dyn std::error::Error>> {
        let api_key = env::var("OPENWEATHER_API_KEY")
            .map_err(|_| "OPENWEATHER_API_KEY has not been set!".to_string())?;

        let (lat, lon) = self.get_loc(&self.city).await?;

        let api_url = format!(
            "https://api.openweathermap.org/data/2.5/forecast/daily?lat={}&lon={}&cnt=7&appid={}",
            lat, lon, api_key
        );

        let resp = reqwest::get(api_url)
            .await?
            .json::<ForecastResponse>()
            .await?;

        let city = &resp.city;

        println!("7-day forecast at {}, {}:", city.name, city.country);

        let to_c = |k: f64| k - 273.15;
        for (day, info) in resp.list.iter().enumerate() {
            let description = info.weather
                .get(0)
                .map(|w| w.description.clone())
                .unwrap_or("N/A".to_string());

            let day = day + 1;
            let temp_min = to_c(info.temp.min);
            let temp_max = to_c(info.temp.max);

            println!("========== Day {} ==========", day);
            println!("Weather     : {}", description);
            println!("Temp min    : {:.1} °C", temp_min);
            println!("Temp max    : {:.1} °C", temp_max);
        }
        println!("============================");
        Ok(())
    }

    async fn get_loc(&self, city: &Option<String>) 
        -> Result<(String, String), Box<dyn std::error::Error>> 
    {
        let api_key = env::var("OPENWEATHER_API_KEY")
            .map_err(|_| "OPENWEATHER_API_KEY has not been set!".to_string())?;

        match city {
            Some(city) => {
                let geocoding_url = format!(
                    "http://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&appid={}",
                    city, api_key
                );
                let resp = reqwest::get(geocoding_url)
                    .await?
                    .json::<Vec<Geocoding>>()
                    .await?;

                match resp.get(0) {
                    Some(loc) => {
                        let (lat, lon) = (
                            loc.lat.clone().to_string(),
                            loc.lon.clone().to_string()
                        );
                        Ok((lat, lon))
                    },
                    None => return Err("No results found.".into())
                }
            },
            None => {
                let ip: IpAddr = reqwest::get("https://api.ipify.org")
                    .await?
                    .text()
                    .await?
                    .parse()
                    .map_err(|_| "Failed to parse IP address".to_string())?;

                let loc = geolocation::find(&ip.to_string())
                    .map_err(|_| "Could not geolocate IP".to_string())?;
                Ok((loc.latitude, loc.longitude))
            }
        }
    }

    pub fn help(&self) {
        println!("===== Command Lists =====");
        println!("- now: weather-now [CITY]");
        println!("- week: weather-week [CITY]");
        println!("=========================");
    }
}