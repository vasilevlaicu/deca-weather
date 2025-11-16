// This table was derived using OpenAI from the National Centers for Environmental Information website:
// https://www.nodc.noaa.gov/archive/arc0021/0002199/1.1/data/0-data/HTML/WMO-CODE/WMO4677.HTM

/// Return the WMO 4677 "present weather" description for a given code (00–99).
/// If the code is outside that range, returns "Unknown WMO weather code".
pub fn describe(code: u32) -> &'static str {
    // using static for immutable strings living in bin.
    match code {
        // 00–19: no precip at station
        0 => "Cloud development not observed or not observable",
        1 => "Clouds generally dissolving or becoming less developed",
        2 => "State of sky on the whole unchanged",
        3 => "Clouds generally forming or developing",
        4 => "Visibility reduced by smoke (e.g. forest fires, industrial smoke, volcanic ash)",
        5 => "Haze",
        6 => "Widespread dust in suspension in the air, not raised by wind at or near the station",
        7 => "Dust or sand raised by wind at or near the station; no duststorm or sandstorm seen",
        8 => "Well-developed dust or sand whirls seen, but no duststorm or sandstorm",
        9 => "Duststorm or sandstorm within sight or at the station during the preceding hour",
        10 => "Mist",
        11 => "Patches of shallow fog or ice fog",
        12 => "More or less continuous shallow fog or ice fog",
        13 => "Lightning visible, no thunder heard",
        14 => "Precipitation within sight, not reaching the ground or sea surface",
        15 => "Precipitation within sight, reaching the ground or sea, but distant (> 5 km)",
        16 => "Precipitation within sight, reaching the ground or sea, near but not at the station",
        17 => "Thunderstorm, but no precipitation at the time of observation",
        18 => {
            "Squalls at or within sight of the station during the preceding hour or at observation"
        }
        19 => "Funnel cloud(s) (tornado or waterspout)",

        // 20–29: precip/fog/thunderstorm in last hour, but not now
        20 => "Drizzle (not freezing) or snow grains, not falling as showers",
        21 => "Rain (not freezing), not falling as showers",
        22 => "Snow, not falling as showers",
        23 => "Rain and snow or ice pellets, not falling as showers",
        24 => "Freezing drizzle or freezing rain, not falling as showers",
        25 => "Shower(s) of rain",
        26 => "Shower(s) of snow, or of rain and snow",
        27 => "Shower(s) of hail, or of rain and hail",
        28 => "Fog or ice fog",
        29 => "Thunderstorm (with or without precipitation)",

        // 30–39: dust/sandstorm, drifting/blowing snow
        30 => "Slight or moderate duststorm or sandstorm, decreasing during the preceding hour",
        31 => "Slight or moderate duststorm or sandstorm, no appreciable change in last hour",
        32 => "Slight or moderate duststorm or sandstorm, begun or increasing during last hour",
        33 => "Severe duststorm or sandstorm, decreasing during the preceding hour",
        34 => "Severe duststorm or sandstorm, no appreciable change in last hour",
        35 => "Severe duststorm or sandstorm, begun or increasing during last hour",
        36 => "Slight or moderate blowing snow, generally low (below eye level)",
        37 => "Heavy drifting snow, generally low (below eye level)",
        38 => "Slight or moderate blowing snow, generally high (above eye level)",
        39 => "Heavy blowing snow, generally high (above eye level)",

        // 40–49: fog or ice fog at the time of observation
        40 => {
            "Fog or ice fog at a distance, not at station in last hour; fog extends above observer"
        }
        41 => "Fog or ice fog in patches",
        42 => "Fog or ice fog, sky visible, becoming thinner during last hour",
        43 => "Fog or ice fog, sky invisible, becoming thinner during last hour",
        44 => "Fog or ice fog, sky visible, no appreciable change during last hour",
        45 => "Fog or ice fog, sky invisible, no appreciable change during last hour",
        46 => "Fog or ice fog, sky visible, has begun or become thicker during last hour",
        47 => "Fog or ice fog, sky invisible, has begun or become thicker during last hour",
        48 => "Fog, depositing rime, sky visible",
        49 => "Fog, depositing rime, sky invisible",

        // 50–59: drizzle
        50 => "Drizzle, not freezing, intermittent, slight at time of observation",
        51 => "Drizzle, not freezing, continuous, slight at time of observation",
        52 => "Drizzle, not freezing, intermittent, moderate at time of observation",
        53 => "Drizzle, not freezing, continuous, moderate at time of observation",
        54 => "Drizzle, not freezing, intermittent, heavy (dense) at time of observation",
        55 => "Drizzle, not freezing, continuous, heavy (dense) at time of observation",
        56 => "Drizzle, freezing, slight",
        57 => "Drizzle, freezing, moderate or heavy (dense)",
        58 => "Drizzle and rain, slight",
        59 => "Drizzle and rain, moderate or heavy",

        // 60–69: rain
        60 => "Rain, not freezing, intermittent, slight at time of observation",
        61 => "Rain, not freezing, continuous, slight at time of observation",
        62 => "Rain, not freezing, intermittent, moderate at time of observation",
        63 => "Rain, not freezing, continuous, moderate at time of observation",
        64 => "Rain, not freezing, intermittent, heavy at time of observation",
        65 => "Rain, not freezing, continuous, heavy at time of observation",
        66 => "Rain, freezing, slight",
        67 => "Rain, freezing, moderate or heavy (dense)",
        68 => "Rain or drizzle and snow, slight",
        69 => "Rain or drizzle and snow, moderate or heavy",

        // 70–79: solid precip, not in showers
        70 => "Intermittent fall of snowflakes, slight at time of observation",
        71 => "Continuous fall of snowflakes, slight at time of observation",
        72 => "Intermittent fall of snowflakes, moderate at time of observation",
        73 => "Continuous fall of snowflakes, moderate at time of observation",
        74 => "Intermittent fall of snowflakes, heavy at time of observation",
        75 => "Continuous fall of snowflakes, heavy at time of observation",
        76 => "Diamond dust (with or without fog)",
        77 => "Snow grains (with or without fog)",
        78 => "Isolated star-like snow crystals (with or without fog)",
        79 => "Ice pellets",

        // 80–99: showery precip, and/or current/recent thunderstorm
        80 => "Rain shower(s), slight",
        81 => "Rain shower(s), moderate or heavy",
        82 => "Rain shower(s), violent",
        83 => "Shower(s) of rain and snow mixed, slight",
        84 => "Shower(s) of rain and snow mixed, moderate or heavy",
        85 => "Snow shower(s), slight",
        86 => "Snow shower(s), moderate or heavy",
        87 => {
            "Shower(s) of snow pellets or small hail, with or without rain or rain and snow, slight"
        }
        88 => {
            "Shower(s) of snow pellets or small hail, with or without rain or rain and snow, moderate or heavy"
        }
        89 => {
            "Shower(s) of hail, with or without rain or rain and snow, not associated with thunder, slight"
        }
        90 => {
            "Shower(s) of hail, with or without rain or rain and snow, not associated with thunder, moderate or heavy"
        }
        91 => "Slight rain at time of observation; thunderstorm during preceding hour but not now",
        92 => {
            "Moderate or heavy rain at time of observation; thunderstorm during preceding hour but not now"
        }
        93 => {
            "Slight snow, or rain and snow mixed, or hail at time of observation; thunderstorm during preceding hour but not now"
        }
        94 => {
            "Moderate or heavy snow, or rain and snow mixed, or hail at time of observation; thunderstorm during preceding hour but not now"
        }
        95 => {
            "Thunderstorm, slight or moderate, without hail but with rain and/or snow at time of observation"
        }
        96 => "Thunderstorm, slight or moderate, with hail at time of observation",
        97 => "Thunderstorm, heavy, without hail but with rain and/or snow at time of observation",
        98 => "Thunderstorm combined with duststorm or sandstorm at time of observation",
        99 => "Thunderstorm, heavy, with hail at time of observation",

        _ => "Unknown WMO weather code",
    }
}

pub fn emoji(code: u32) -> &'static str {
    match code {
        0..=3 => "☁️",
        45 | 48 => "🌫️",
        51..=55 => "🌦️",
        61..=65 => "🌧️",
        71..=75 => "❄️",
        80..=82 => "🌧️",
        95..=99 => "⛈️",
        _ => "?",
    }
}
