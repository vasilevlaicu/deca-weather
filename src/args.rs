use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct WeatherArgs {
    #[clap(subcommand)]
    pub command: Option<Commands>, // Option to make it work with None too.
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Show daily weather forcast for our list of favourite Belgian cities.
    List,
    /// Get weather of a registered favourite Belgian city.
    Get(GetArgs),

    /// Add a city to the database (sync).
    AddCity(AddCityArgs),

    /// Remove city from the database.
    RemoveCity(RemoveCityArgs),

    /// Get weather forecast for a city in the database.
    GetDb(GetDbArgs),
}

#[derive(Args, Debug)]
pub struct GetArgs {
    /// Name of the city.
    pub city: String,

    /// Include forcast for tomorrow.
    #[arg(long)] // -- flag for tomorrow
    pub tomorrow: bool,

    /// Include forcast for the day after tomorrow.
    #[arg(long)] // -- flag for day after tomorrow
    pub day_after: bool,
}

#[derive(Args, Debug)]
pub struct AddCityArgs {
    /// Name of the city to add
    pub city: String,
}

#[derive(Args, Debug)]
pub struct RemoveCityArgs {
    /// Name of the city to remove
    pub city: String,
}

#[derive(Args, Debug)]
pub struct GetDbArgs {
    /// Name of the city to get
    pub city: String,

    /// Include forcast for tomorrow.
    #[arg(long)] // -- flag for tomorrow
    pub tomorrow: bool,

    /// Include forcast for the day after tomorrow.
    #[arg(long)] // -- flag for day after tomorrow
    pub day_after: bool,
}
