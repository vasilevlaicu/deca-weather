use anyhow::Result;
use clap::Parser;
use deca_weather::args::{AddCityArgs, Commands, GetArgs, GetDbArgs, RemoveCityArgs, WeatherArgs};
use deca_weather::cities::get_favourite_cities;
use deca_weather::db::Db;
use deca_weather::models::City;
use deca_weather::open_meteo::{get_city_forecast, get_geocode};

#[tokio::main]
async fn main() -> Result<()> {
    let args = WeatherArgs::parse();

    let mut db = Db::open("deca-weather.db")?;

    preload_favourites(&db)?;

    match args.command {
        None | Some(Commands::List) => {
            handle_list(&mut db).await?;
        }
        Some(Commands::Get(get_args)) => {
            handle_get(&mut db, &get_args).await?;
        }
        Some(Commands::AddCity(add_args)) => {
            handle_add_city(&mut db, &add_args).await?;
        }
        Some(Commands::RemoveCity(remove_args)) => {
            handle_remove_city(&mut db, &remove_args)?;
        }
        Some(Commands::GetDb(get_db_args)) => {
            handle_get_db(&db, &get_db_args).await?;
        }
    }

    Ok(())
}

/// Task 1: Prints the daily forecast of our 10 favourite Belgian cities
// TODO: I don't know what we want to do with the hardcoded list. We could set favourites in the DB etc.
// One thing we could do now is fetch favourites from the api only if they're not in the db.
async fn handle_list(db: &mut Db) -> Result<()> {
    println!("Daily weather for your favourite Belgian cities: \n");

    for city in get_favourite_cities() {
        let forecast = get_city_forecast(&city).await?;
        // we just want today so [0]
        forecast.print_days_for_city(&city, &[0]);
        println!();

        // add the fetched forecasts in our db:
        db.insert_city(&city)?; // city probably already inside thanks to the preload
        db.save_forecast_for_city(&city, &forecast)?;
    }
    Ok(())
}

/// Task 2: Get forecast (optionally for tomorrow and the day after) for your favourite city
async fn handle_get(db: &mut Db, args: &GetArgs) -> Result<()> {
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
    let city = get_city(db, &args.city).await?;
    let forecast = get_city_forecast(&city).await?;

    forecast.print_days_for_city(&city, &days);
    println!();

    // add the fetched forecasts in our db:
    db.insert_city(&city)?;
    db.save_forecast_for_city(&city, &forecast)?;

    Ok(())
}

/// Task 3: Get a City struct from a city name
pub async fn get_city(db: &Db, name: &str) -> Result<City> {
    // Try from favourites
    if let Some(city) = db.get_city_by_name(name)? {
        println!("'{}' found in local database!", city.name);
        return Ok(city);
    }
    /*if let Some(city) = get_favourite_cities()
        .into_iter()
        .find(|c| c.name.eq_ignore_ascii_case(name))
    {
        println!("'{}' found inside our list of favourites!", city.name);
        return Ok(city);
    }*/

    // Try using the geocoding api
    println!(
        "'{}' not in database. Searching for geocode online...\n",
        name
    );

    let response = get_geocode(name).await?;
    let city = City::try_from(response)?;
    Ok(city)
}

/// Task 3: Preload our favourite cities from our hardcoded vec.
// If already in the db, it'll just do nothing
fn preload_favourites(db: &Db) -> Result<()> {
    for city in get_favourite_cities() {
        db.insert_city(&city)?;
    }

    Ok(())
}

/// Task 3: Add city to the database
async fn handle_add_city(db: &mut Db, args: &AddCityArgs) -> Result<()> {
    println!("Adding city '{}' to the database...\n", args.city);
    let city = get_city(db, &args.city).await?;
    db.insert_city(&city)?;

    let forecast = get_city_forecast(&city).await?;

    db.save_forecast_for_city(&city, &forecast)?;

    println!("City added to database: {}", city);
    Ok(())
}

/// Task 3: Remove a city from the DB (and all related entries)
fn handle_remove_city(db: &mut Db, args: &RemoveCityArgs) -> Result<()> {
    println!("Removing city '{}' from the local database...\n", args.city);

    let removed = db.delete_city(&args.city)?;
    if removed == 0 {
        println!("No city named '{}' was found in the database.", args.city);
    } else {
        println!(
            "Removed '{}' and all related entries from the database.",
            args.city
        );
    }

    Ok(())
}

/// Task 3: Get weather forecast for a city in the database
async fn handle_get_db(db: &Db, args: &GetDbArgs) -> Result<()> {
    println!("Fetching forecast for '{}' from the database.\n", args.city);

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

    let city = match db.get_city_by_name(&args.city)? {
        Some(c) => c,
        None => {
            println!("City '{}' is not in the local database.", args.city);
            println!("Run 'get' or 'add-city' first to fetch and store it.");
            return Ok(());
        }
    };
    let rows_from_db = db.get_daily_forecast_for_city(&city)?;

    if rows_from_db.is_empty() {
        println!(
            "No stored forecast found for '{}' in the database.",
            args.city
        );
        println!("Run the get command first to fetch and store the forecast.");
        return Ok(());
    }
    db.print_days_for_city(&city, &rows_from_db, &days);
    Ok(())
}
