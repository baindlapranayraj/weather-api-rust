use dotenv::dotenv;
use reqwest::Error;
use serde::Deserialize;
use std::{env, io};

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: i32,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
}

#[tokio::main]
async fn fetch_data(city: String, api_key: String) -> Result<WeatherResponse, Error> {
    let api_url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city, api_key
    );

    let response = reqwest::get(&api_url).await?;
    let data: WeatherResponse = response.json().await?;

    Ok(data)
}

fn main() {
    println!("Weather APP Lets get started");
    dotenv().ok();

    loop {
        let api_key = match env::var("API_KEY") {
            Ok(key) => key,
            Err(e) => {
                println!("{e}");
                "key is not available".to_string()
            }
        };
        let mut city_name = String::new();
        println!("Enter tha name of the country: ");
        io::stdin()
            .read_line(&mut city_name)
            .expect("Invalid input try again");
        let city_name = city_name.trim().to_lowercase();
        match fetch_data(city_name, api_key) {
            Ok(data) => {
                let weather_data = data.weather.get(0);
                let weather_description = match weather_data {
                    None => {
                        let desc = "No Description".to_string();
                        desc
                    }
                    Some(data) => {
                        let weather_data = data.description.to_string();
                        weather_data
                    }
                };
                let main = data.main;
                let wind_speed = data.wind.speed;

                println!(
                    "The weather is {},
                    the temperature and humidity is {}^C and {}%
                    finally the speed of wind currently is {}m/s",
                    weather_description, main.temp, main.humidity, wind_speed
                );
            }
            Err(e) => println!("Error occured {}", e),
        }

        println!("Do you want to continue:");
        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Invalid input try again");
        let option = option.trim().to_lowercase();

        match option == String::from("yes") {
            true => {
                continue;
            }
            false => {
                break;
            }
        }
    }
}

// {
//   weather: [Weather { description: "broken clouds" }],
//   main: Main { temp: 292.65, humidity: 74 },
//   wind: Wind { speed: 4.63 }
// }
