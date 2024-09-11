use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Reindeer {
    name: String,
    strength: i32,
    speed: Option<f32>,
    height: Option<i32>,
    antler_width: Option<i32>,
    snow_magic_power: Option<i32>,
    favorite_food: Option<String>,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: Option<i32>,
}

#[derive(Serialize)]
struct ContestResult {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

impl ContestResult {
    pub fn get_contest_results(reindeers: &Vec<Reindeer>) -> Self {
        ContestResult {
            fastest: Self::get_fastest(reindeers),
            tallest: Self::get_tallest(reindeers),
            magician: Self::get_magician(reindeers),
            consumer: Self::get_consumer(reindeers),
        }
    }
    fn get_fastest(reindeers: &Vec<Reindeer>) -> String {
        let mut fastest_reindeer = reindeers.iter().next().unwrap();
        for reindeer in reindeers.iter().skip(1) {
            if reindeer.speed > fastest_reindeer.speed {
                fastest_reindeer = reindeer;
            }
        }
        format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest_reindeer.strength, fastest_reindeer.name
        )
    }

    fn get_tallest(reindeers: &Vec<Reindeer>) -> String {
        let mut tallest_reindeer = reindeers.iter().next().unwrap();
        for reindeer in reindeers.iter().skip(1) {
            if reindeer.height > tallest_reindeer.height {
                tallest_reindeer = reindeer;
            }
        }
        format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest_reindeer.name,
            tallest_reindeer.antler_width.unwrap()
        )
    }

    fn get_magician(reindeers: &Vec<Reindeer>) -> String {
        let mut magician_reindeer = reindeers.iter().next().unwrap();
        for reindeer in reindeers.iter().skip(1) {
            if reindeer.snow_magic_power > magician_reindeer.snow_magic_power {
                magician_reindeer = reindeer;
            }
        }
        format!(
            "{} could blast you away with a snow magic power of {}",
            magician_reindeer.name,
            magician_reindeer.snow_magic_power.unwrap()
        )
    }

    fn get_consumer(reindeers: &Vec<Reindeer>) -> String {
        let mut consumer_reindeer = reindeers.iter().next().unwrap();
        for reindeer in reindeers.iter().skip(1) {
            if reindeer.candies_eaten_yesterday > consumer_reindeer.candies_eaten_yesterday {
                consumer_reindeer = reindeer;
            }
        }
        format!(
            "{} ate lots of candies, but also some {}",
            consumer_reindeer.name,
            consumer_reindeer.favorite_food.clone().unwrap()
        )
    }
}

async fn calculate_strength(Json(reindeers): Json<Vec<Reindeer>>) -> String {
    let mut total_strength = 0;
    for reindeer in reindeers {
        total_strength += reindeer.strength;
    }
    total_strength.to_string()
}

async fn get_contest_results(Json(reindeers): Json<Vec<Reindeer>>) -> Json<ContestResult> {
    Json(ContestResult::get_contest_results(&reindeers))
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/4/strength", post(calculate_strength))
        .route("/4/contest", post(get_contest_results))
}
