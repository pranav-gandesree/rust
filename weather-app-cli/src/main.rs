use std::io;
use serde::Deserialize; 
use dotenv::dotenv; 
use std::env;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>, 
    main: Main, 
    wind: Wind, 
    name: String,
}


#[derive(Deserialize, Debug)]
struct Weather {
    description: String, 
}


#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64, 
    pressure: f64, 
}


#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64, 
}


fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {

    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );

    
    let response = reqwest::blocking::get(&url)?;

    let response_json = response.json::<WeatherResponse>()?;
    Ok(response_json) 
}


fn display_weather_info(response: &WeatherResponse) {
    
    let description = &response.weather[0].description;
    let temperature = response.main.temp;
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;


    let weather_text = format!(
        "Weather in {}: {} {}
> Temperature: {:.1}°C, 
> Humidity: {:.1}%, 
> Pressure: {:.1} hPa, 
> Wind Speed: {:.1} m/s",
        response.name,
        description,
        temperature,
        humidity,
        pressure,
        wind_speed,
    );
}



fn main() {
    
    dotenv().ok();

    loop {
        println!("{}", "enter the name of the city:"); 
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("failed to read input"); 
        let city = city.trim();

        println!("{}", "enter the country code"); 

        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("failed to read input"); 
        let country_code = country_code.trim();

        
        let api_key = env::var("OPENWEATHER_API_KEY").expect("API key not found");

      
        match get_weather_info(&city, &country_code, &api_key) {
            Ok(response) => {
                display_weather_info(&response); 
            }
            Err(err) => {
                eprintln!("Error: {}", err); 
            }
        }

        println!("{}", "Do you want to search for weather in another city? (yes/no):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read input");
        let input = input.trim().to_lowercase();

        if input != "yes" {
            println!(".....");
            break; 
        }
    }
}
