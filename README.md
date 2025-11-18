# Weather App

This project aims to build a cli weather app in rust using the open-meteo api.

## Tasks Checklist

### Task 1:

- [x] Create a local application that can be run & parse args.
- [x] Get the daily weather forecast for 10 favourite Belgian cities (hardcoded geocode).
- [x] Print their forecast in the terminal.
- [x] **Extra:** WMO weather-code descriptions
- [x] **Extra:** Card style formatting for terminal prints.
- [x] **Extra:** Built task 1 with reusable code and structures for task 2

### Option<Task 2>:

- [x] Select one city from the list of 10 to get its weather forecast.
- [x] Get the weather forecast for an unknown city (using open-meteo geocoding API?).
- [x] Add a command to get the weather forecast for tomorrow.
- [x] Add a command to get the weather forecast for the day after tomorrow.
- [x] Add a command for tomorrow & day after tomorrow at the same time. 

### Option<Task 3>:

- [x] Store application results in a local Sqlite3 database.
- [x] Add a new city to synchronize.
- [x] Get the weather forecast for a city available in the DB.
- [x] Remove a city from the DB.