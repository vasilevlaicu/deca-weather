use clap::{Parser, Subcommand};

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
}
