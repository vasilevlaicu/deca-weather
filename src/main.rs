use anyhow::Result;
use clap::Parser;
use deca_weather::args::{Commands, GetArgs, WeatherArgs};
use deca_weather::cities::get_favourite_cities;
use deca_weather::models::City;
use deca_weather::open_meteo::{get_city_forecast, get_geocode};

#[tokio::main]
async fn main() -> Result<()> {
    let args = WeatherArgs::parse();

    match args.command {
        None | Some(Commands::List) => {
            handle_list().await?;
        }
        Some(Commands::Get(get_args)) => {
            handle_get(&get_args).await?;
        }
    }

    Ok(())
}

/// Task 1: Prints the daily forecast of our 10 favourite Belgian cities
async fn handle_list() -> Result<()> {
    println!("Daily weather for your favourite Belgian cities: \n");

    for city in get_favourite_cities() {
        let forecast = get_city_forecast(&city).await?;
        // we just want today so [0]
        forecast.print_days_for_city(&city, &[0]);
        println!();
    }
    Ok(())
}

/// Task 2: Get forecast (optionally for tomorrow and the day after) for your favourite city
async fn handle_get(args: &GetArgs) -> Result<()> {
    println!(
        "Get forecast (optionally for tomorrow and the day after) for your favourite city: \n"
    );
    let mut days = vec![];
    if args.tomorrow {
        days.push(1);
    }
    if args.day_after {
        days.push(2);
    }
    if !args.tomorrow && !args.day_after {
        days.push(0);
    }
    let city = get_city(&args.city).await?;
    let forecast = get_city_forecast(&city).await?;

    forecast.print_days_for_city(&city, &days);
    println!();
    Ok(())
}

pub async fn get_city(name: &str) -> Result<City> {
    // Try from favourites
    if let Some(city) = get_favourite_cities()
        .into_iter()
        .find(|c| c.name.eq_ignore_ascii_case(name))
    {
        println!("'{}' found inside our list of favourites!", city.name);
        return Ok(city);
    }

    // Try using the geocoding api
    println!("'{}' not in favourites. Searching online...\n", name);

    let response = get_geocode(name).await?;
    let city = City::try_from(response)?;
    Ok(city)
}
