use serde::{Deserialize, Serialize};
use std::env;

struct Args {
    units: Units,
    city: String,
}

enum Units {
    Metric,
    Imperial,
}

impl Units {
    fn display(&self, value: f32) -> String {
        match self {
            Self::Metric => format!("{}°C", value),
            Self::Imperial => format!("{}°F", value),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::Metric => String::from("metric"),
            Self::Imperial => String::from("imperial"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Coordinates {
    lon: f32,
    lat: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f32,
    deg: i32,
    gust: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Main {
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    pressure: f32,
    humidity: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiResult {
    coord: Coordinates,
    weather: Vec<Weather>,
    base: String,
    name: String,
    wind: Wind,
    main: Main,
}

fn parse_units(units: &str) -> Result<Units, String> {
    match units {
        "metric" => Ok(Units::Metric),
        "imperial" => Ok(Units::Imperial),
        _ => Err("unhandled metric".to_string()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let weather_api_key = env::var("WEATHER_APP_KEY").expect("WEATHER_APP_KEY variable is not set");
    let mut args = pico_args::Arguments::from_env();
    let args = Args {
        units: args
            .opt_value_from_fn("--units", parse_units)?
            .unwrap_or(Units::Metric),
        city: args
            .opt_value_from_str("--city")?
            .expect("City params required"),
    };

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units={}",
        args.city,
        weather_api_key,
        args.units.to_string()
    );

    let resp = reqwest::get(&url).await?.text().await?;
    let apiresult: ApiResult = serde_json::from_str(&resp).unwrap();

    println!("\n-----------------------");
    println!("{}", apiresult.name);
    println!("---");
    println!("Temperature : {}", args.units.display(apiresult.main.temp));
    println!(
        "Min : {}   /  Max : {}",
        args.units.display(apiresult.main.temp_min),
        args.units.display(apiresult.main.temp_max)
    );
    println!(
        "Feels like : {}",
        args.units.display(apiresult.main.feels_like)
    );
    println!("-----------------------");
    Ok(())
}
