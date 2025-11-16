use anyhow::Result;
use clap::Parser;
use deca_weather::args::{Commands, WeatherArgs};
use deca_weather::cities::get_favourite_cities;
use deca_weather::open_meteo::get_city_forecast;
#[tokio::main]
async fn main() -> Result<()> {
    let args = WeatherArgs::parse();

    match args.command {
        None | Some(Commands::List) => {
            handle_list().await?;
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
