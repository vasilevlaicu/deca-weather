use crate::models::City;

// lat and long taken from the open-meteo geocoding api: https://open-meteo.com/en/docs/geocoding-api?name=Dinant
// could use static vec to avoid memory reallocation?
pub fn get_favourite_cities() -> Vec<City> {
    vec![
        City {
            name: "Brussels".to_string(),
            lat: 50.85045,
            long: 4.34878,
        },
        City {
            name: "Antwerp".to_string(),
            lat: 51.22047,
            long: 4.40026,
        },
        City {
            name: "Grimbergen".to_string(),
            lat: 50.93409,
            long: 4.37213,
        },
        City {
            name: "Liège".to_string(),
            lat: 50.63373,
            long: 5.56749,
        },
        City {
            name: "Louvain-la-Neuve".to_string(),
            lat: 50.66829,
            long: 4.61443,
        },
        City {
            name: "Waterloo".to_string(),
            lat: 50.71469,
            long: 4.3991,
        },
        City {
            name: "Bruges".to_string(),
            lat: 51.20892,
            long: 3.22424,
        },
        City {
            name: "Leveun".to_string(),
            lat: 50.87959,
            long: 4.70093,
        },
        City {
            name: "Knokke-Heist".to_string(),
            lat: 51.35,
            long: 3.26667,
        },
        City {
            name: "Dinant".to_string(),
            lat: 50.25807,
            long: 4.91166,
        },
    ]
}
