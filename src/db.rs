use crate::models::{City, ForecastResponse};
use crate::wmo::{describe, emoji};
use anyhow::{Context, Result};
use chrono::{Duration, Local, NaiveDate};
use rusqlite::{Connection, OptionalExtension, params};

pub struct Db {
    conn: Connection,
}

/// Struct row type returned by the db
pub struct DbDailyForecastRow {
    pub date: String,
    pub weather_code: i64,
    pub t_min: f64,
    pub t_max: f64,
    pub t_mean: f64,
}

impl Db {
    /// open connection to the db
    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path)
            .with_context(|| format!("Failed to open SQlite database at {}", path))?;
        let db = Db { conn };
        db.init_schema()?;
        Ok(db)
    }

    /// Create the table schema of the db
    fn init_schema(&self) -> Result<()> {
        self.conn.execute("PRAGMA foreign_keys = ON;", [])?;

        // Creating cities table where the name, lat, long tuple is unique
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS cities (
                id    INTEGER PRIMARY KEY AUTOINCREMENT,
                name  TEXT NOT NULL COLLATE NOCASE,
                lat   REAL NOT NULL,
                long  REAL NOT NULL,
                UNIQUE (name, lat, long)
            );",
            [],
        )?;

        // Creating daily forecasts table with city_id as foreign key
        // and cascading deletes when we remove a city from the cities table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS daily_forecasts (
                city_id      INTEGER NOT NULL,
                date         TEXT NOT NULL,
                weather_code INTEGER NOT NULL,
                t_min        REAL NOT NULL,
                t_max        REAL NOT NULL,
                t_mean       REAL NOT NULL,
                fetched_at   TEXT NOT NULL,
                PRIMARY KEY (city_id, date),
                FOREIGN KEY (city_id) REFERENCES cities(id) ON DELETE CASCADE
            );",
            [],
        )?;
        Ok(())
    }

    /// Insert a City struct in the db
    pub fn insert_city(&self, city: &City) -> Result<()> {
        self.conn.execute(
            "
            INSERT INTO cities (name, lat, long)
            VALUES (?1, ?2, ?3)
            ON CONFLICT(name, lat, long) DO NOTHING
            ",
            params![city.name, city.lat, city.long],
        )?;
        Ok(())
    }

    /// Delete a city and all its records from the db
    pub fn delete_city(&self, name: &str) -> Result<usize> {
        let count = self
            .conn
            .execute("DELETE FROM cities WHERE name = ?1", params![name])?;
        Ok(count)
    }

    /// Get a list of registered cities (name, lat, long) from the cities table.
    pub fn list_cities(&self) -> Result<Vec<City>> {
        let mut statement = self
            .conn
            .prepare("SELECT name, lat, long FROM cities ORDER BY name")?;
        let rows = statement.query_map([], |row| {
            Ok(City {
                name: row.get(0)?,
                lat: row.get(1)?,
                long: row.get(2)?,
            })
        })?;

        let mut output = vec![];
        for row in rows {
            output.push(row?);
        }
        Ok(output)
    }

    /// Helper to get city id for forcast insertion
    fn get_city_id(&self, city: &City) -> Result<Option<i64>> {
        let id = self
            .conn
            .query_row(
                "SELECT id FROM cities 
                 WHERE name = ?1 AND lat = ?2 AND long = ?3
                ",
                params![city.name, city.lat, city.long],
                |row| row.get(0),
            )
            .optional()?;
        Ok(id)
    }

    /// Save a City ForecastResponse in the db
    pub fn save_forecast_for_city(
        &mut self,
        city: &City,
        forecast: &ForecastResponse,
    ) -> Result<()> {
        let city_id = self
            .get_city_id(city)?
            .context("City must exist in database before adding a forecast.")?;
        let days = forecast.len();
        let dates = &forecast.daily.time;
        let weather_codes = &forecast.daily.weather_code;
        let t_mins = &forecast.daily.temperature_2m_min;
        let t_maxs = &forecast.daily.temperature_2m_max;
        let t_means = &forecast.daily.temperature_2m_mean;

        let tx = self.conn.transaction()?;

        for day in 0..days {
            tx.execute(
                "INSERT INTO daily_forecasts (
            city_id, date, weather_code, t_min, t_max, t_mean, fetched_at
          ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, datetime('now'))
           ON CONFLICT(city_id, date) DO UPDATE SET
           weather_code = excluded.weather_code, 
           t_min = excluded.t_min,
           t_max = excluded.t_max,
           t_mean = excluded.t_mean,
           fetched_at = excluded.fetched_at
          ",
                params![
                    city_id,
                    dates[day],
                    weather_codes[day],
                    t_mins[day],
                    t_maxs[day],
                    t_means[day]
                ],
            )?;
        }

        tx.commit()?;
        Ok(())
    }

    /// Get all daily forecasts for a city from the db.
    pub fn get_daily_forecast_for_city(&self, city: &City) -> Result<Vec<DbDailyForecastRow>> {
        let city_id = self
            .get_city_id(city)?
            .context("City not found in the DataBase")?;

        let mut statement = self.conn.prepare(
            "SELECT date, weather_code, t_min, t_max, t_mean FROM daily_forecasts 
            WHERE city_id = ?1
            ORDER BY date
            ",
        )?;

        let rows = statement.query_map(params![city_id], |row| {
            Ok(DbDailyForecastRow {
                date: row.get(0)?,
                weather_code: row.get(1)?,
                t_min: row.get(2)?,
                t_max: row.get(3)?,
                t_mean: row.get(4)?,
            })
        })?;
        let mut output = vec![];
        for row in rows {
            output.push(row?);
        }
        Ok(output)
    }

    /// Get City struct from db using a &str name
    pub fn get_city_by_name(&self, name: &str) -> Result<Option<City>> {
        let mut stmt = self.conn.prepare(
            "SELECT name, lat, long
             FROM cities
             WHERE name = ?1 COLLATE NOCASE
             LIMIT 1",
        )?;

        let city = stmt
            .query_row(params![name], |row| {
                Ok(City {
                    name: row.get(0)?,
                    lat: row.get(1)?,
                    long: row.get(2)?,
                })
            })
            .optional()?;

        Ok(city)
    }

    /// Pretty print daily forcasts for offset = days starting with 0 for today.
    pub fn print_days_for_city(
        &self,
        city: &City,
        rows: &[DbDailyForecastRow],
        offsets: &[i64], // 0 = today, 1 = tomorrow, etc.
    ) {
        if rows.is_empty() || offsets.is_empty() {
            println!("No forecast data available for {}", city.name);
            return;
        }

        let today = Local::now().date_naive();

        // If all rows are before today
        let max_stored_date = rows
            .iter()
            .filter_map(|r| NaiveDate::parse_from_str(&r.date, "%Y-%m-%d").ok())
            .max();

        if let Some(max_date) = max_stored_date {
            if max_date < today {
                println!("Stored forecast for '{}' is outdated.", city.name);
                println!("Last stored day: {max_date}");
                println!("Run 'get {}' first to refresh the forecast.\n", city.name);
                return;
            }
        }

        // No data for any requested offset (today / tomorrow / D+N)
        let has_any_requested = offsets.iter().any(|&offset| {
            let target_date = today + Duration::days(offset);
            let target_str = target_date.to_string(); // "YYYY-MM-DD"
            rows.iter().any(|r| r.date == target_str)
        });

        if !has_any_requested {
            println!(
                "No stored forecast for the requested day(s) for '{}'.",
                city.name
            );
            println!(
                "Run 'get {}' or 'add-city Brussels' first to fetch and store the latest forecast.\n",
                city.name
            );
            return;
        }

        // City header (same style as ForecastResponse::print_days_for_city)
        let header = city.to_string();
        let width = usize::max(45, header.len() + 4);
        let thick_line = "═".repeat(width);
        let line = "─".repeat(width);
        let spaces = " ".repeat(width - header.chars().count() - 1);

        println!("\n╔{thick_line}╗");
        println!("║ {header}{spaces}║");
        println!("╚{thick_line}╝");

        for &offset in offsets {
            let target_date = today + Duration::days(offset);
            let target_str = target_date.to_string(); // "YYYY-MM-DD"

            let label = match offset {
                0 => "Today".to_string(),
                1 => "Tomorrow".to_string(),
                n => format!("D+{n}"),
            };

            if let Some(row) = rows.iter().find(|r| r.date == target_str) {
                let date = &row.date;
                let code = row.weather_code as u32;
                let icon = emoji(code);
                let desc = describe(code);

                let t_min = row.t_min;
                let t_max = row.t_max;
                let t_mean = row.t_mean;

                let u = "°C";

                println!("   [{label}] {date}");
                println!("   {icon}  {desc}");
                println!("   🌡️ Min:   {t_min:.1} {u}");
                println!("   🌡️ Max:  {t_max:.1} {u}");
                println!("   🌡️ Mean: {t_mean:.1} {u}");
                println!("{}", line);
            } else {
                println!("   [{label}] {target_str}");
                println!("   No data stored in DB for this date.");
                println!("{}", line);
            }
        }
    }
}
