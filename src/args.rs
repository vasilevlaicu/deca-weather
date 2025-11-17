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
