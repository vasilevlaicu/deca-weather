use anyhow::{Context, Result, bail};

use crate::models::{City, ForecastResponse, GeoResponse};

// https://api.open-meteo.com/v1/forecast?latitude=50.8505&longitude=4.3488&daily=weather_code,temperature_2m_min,temperature_2m_max,temperature_2m_mean&timezone=auto

const OPEN_METEO_BASE_URL: &str = "https://api.open-meteo.com/v1/forecast";
const OPEN_GEOCODE_BASE_URL: &str = "https://geocoding-api.open-meteo.com/v1/search";

/// Helper to get city forecast by using City struct directly
pub async fn get_city_forecast(city: &City) -> Result<ForecastResponse> {
    get_forecast(city.lat, city.long)
        .await
        .with_context(|| format!("Failed to fetch forecast for city {}", city.name))
}

/// Get city forecast by geocode (lat/long)
pub async fn get_forecast(lat: f64, long: f64) -> Result<ForecastResponse> {
    // building params for the query (list of tuples)
    let params = [
        ("latitude", lat.to_string()),
        ("longitude", long.to_string()),
        (
            "daily",
            "weather_code,temperature_2m_min,temperature_2m_max,temperature_2m_mean".to_string(),
        ),
        ("timezone", "auto".to_string()),
    ];

    let response = reqwest::Client::new()
        .get(OPEN_METEO_BASE_URL)
        .query(&params)
        .send()
        .await
        .context("Failed to send request to open-meteo")?;

    let status = response.status();
    if !status.is_success() {
        bail!("No success: Opent-meteo returned: {status}");
    }

    let forecast = response
        .json::<ForecastResponse>()
        .await
        .context("Couldn't deserialize open-meteo response")?;

    Ok(forecast)
}

// https://geocoding-api.open-meteo.com/v1/search?name=Grimbergen&count=1&language=en

/// Get city geocoding info (lat/long)
pub async fn get_geocode(name: &str) -> Result<GeoResponse> {
    // building params for the query (list of tuples)
    let params = [("name", name.trim()), ("count", "1"), ("language", "en")];

    let response = reqwest::Client::new()
        .get(OPEN_GEOCODE_BASE_URL)
        .query(&params)
        .send()
        .await
        .context("Failed to send request to open-meteo geocoding api")?;

    let status = response.status();
    if !status.is_success() {
        bail!("No success: Opent-meteo geocoding returned: {status}");
    }

    let geocode = response
        .json::<GeoResponse>()
        .await
        .context("Couldn't deserialize open-meteo geocoding response")?;

    Ok(geocode)
}
