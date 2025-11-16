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

- [ ] Select one city from the list of 10 to get its weather forecast.
- [ ] Get the weather forecast for an unknown city (using open-meteo geocoding API?).
- [ ] Add a command to get the weather forecast for tomorrow.
- [ ] Add a command to get the weather forecast for the day after tomorrow.
- [ ] Add a command for tomorrow & day after tomorrow at the same time. 

### Option<Task 3>:

- [ ] Store application results in a local Sqlite3 database.
- [ ] Add a new city to synchronize.
- [ ] Get the weather forecast for a city available in the DB.
- [ ] Remove a city from the DB.

### Additional features
- [ ] Use axum to make an API/interface to easily use the open-meteo api.
- [ ] Create a weather app frontend to use it. 