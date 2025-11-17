use crate::wmo::{describe, emoji};
use anyhow::{Error, bail};
use serde::Deserialize;
use std::fmt;

/// City with coordinates
#[derive(Debug, Clone)]
pub struct City {
    pub name: String,
    pub lat: f64,
    pub long: f64,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({:.4}, {:.4})", self.name, self.lat, self.long)
    }
}

// Structs to deserialize open-meteo forecast api results based of this kind of response:
// https://api.open-meteo.com/v1/forecast?latitude=50.8505&longitude=4.3488&daily=weather_code,temperature_2m_min,temperature_2m_max,temperature_2m_mean&timezone=auto

#[derive(Debug, Deserialize)]
pub struct ForecastResponse {
    pub daily: DailyForecast,
    pub daily_units: DailyUnits,
}

#[derive(Debug, Deserialize)]
pub struct DailyUnits {
    pub time: String,
    pub temperature_2m_max: String,
    pub temperature_2m_min: String,
    pub weather_code: String,
    pub temperature_2m_mean: String,
}

#[derive(Debug, Deserialize)]
pub struct DailyForecast {
    pub time: Vec<String>,
    pub temperature_2m_max: Vec<f64>,
    pub temperature_2m_min: Vec<f64>,
    pub weather_code: Vec<u32>,
    pub temperature_2m_mean: Vec<f64>,
}

impl ForecastResponse {
    pub fn len(&self) -> usize {
        self.daily.time.len()
    }

    // Formatting code to build cards (labels & emoji idea from gpt)
    pub fn print_days_for_city(&self, city: &City, indices: &[usize]) {
        if self.len() == 0 || indices.is_empty() {
            println!("No forecast data available for {}", city.name);
            return;
        }

        // City header
        let header = city.to_string();
        let width = usize::max(45, header.len() + 4);
        let thick_line = "═".repeat(width);
        let line = "─".repeat(width);
        // just the spaces between the city print and right side of the box
        let spaces = " ".repeat(width - header.chars().count() - 1);

        println!("\n╔{thick_line}╗");
        println!("║ {header}{spaces}║");
        println!("╚{thick_line}╝");

        for &id in indices {
            if id >= self.len() {
                println!("  [D+{id}] No data");
                continue;
            }

            let date = &self.daily.time[id];
            let code = self.daily.weather_code[id];
            let icon = emoji(code);
            let desc = describe(code);

            let t_min = self.daily.temperature_2m_min[id];
            let t_max = self.daily.temperature_2m_max[id];
            let t_mean = self.daily.temperature_2m_mean[id];

            let u = &self.daily_units.temperature_2m_min; // all temps use celsius 

            // Label (Today, D+1, D+2 etc)
            let label = match id {
                0 => "Today".to_string(),
                1 => "Tomorrow".to_string(),
                n => format!("D+{n}"),
            };

            // Card
            println!("   [{label}] {date}");
            println!("   {icon}  {desc}");
            println!("   🌡️ Min:   {t_min:.1} {u}");
            println!("   🌡️ Max:  {t_max:.1} {u}");
            println!("   🌡️ Mean: {t_mean:.1} {u}");

            // long line after the end of a day
            println!("{}", line);
        }
    }
}

// Structs to deserialize open-meteo geocoding api results based of this kind of response:
// https://geocoding-api.open-meteo.com/v1/search?name=Grimbergen&count=1&language=en&format=json

#[derive(Debug, Deserialize)]
pub struct GeoResponse {
    pub results: Option<Vec<GeoResult>>,
}

#[derive(Debug, Deserialize)]
pub struct GeoResult {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl TryFrom<GeoResponse> for City {
    type Error = Error;
    fn try_from(geo_response: GeoResponse) -> Result<Self, Self::Error> {
        let results = match geo_response.results {
            Some(v) if !v.is_empty() => v,
            _ => bail!("No geocoding results found"),
        };

        let city_info = &results[0];

        Ok(City {
            name: city_info.name.to_string(),
            lat: city_info.latitude,
            long: city_info.longitude,
        })
    }
}
