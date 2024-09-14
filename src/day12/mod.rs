use std::{
    collections::HashMap,
    str::FromStr,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use ulid::Ulid;
use uuid::Uuid;

use crate::error::AppError;

#[derive(Clone, Debug)]
struct AppState {
    multi_stopwatch: HashMap<String, SystemTime>,
}

type UlidRequest = Vec<String>;
type SharedState = Arc<Mutex<AppState>>;

async fn save_packet(
    State(state): State<SharedState>,
    Path(packet_id): Path<String>,
) -> Result<(), AppError> {
    state
        .lock()
        .unwrap()
        .multi_stopwatch
        .entry(packet_id)
        .and_modify(|e| *e = SystemTime::now())
        .or_insert(SystemTime::now());
    Ok(())
}

async fn load_packet(
    State(state): State<SharedState>,
    Path(packet_id): Path<String>,
) -> Result<String, AppError> {
    let time_diff = if let Some(&timestamp) = state.lock().unwrap().multi_stopwatch.get(&packet_id)
    {
        SystemTime::now().duration_since(timestamp)?
    } else {
        Duration::ZERO
    };
    Ok(time_diff.as_secs().to_string())
}

async fn ulids(Json(ulids): Json<UlidRequest>) -> Result<Json<Vec<String>>, AppError> {
    let ulids = ulids
        .iter()
        .flat_map(|s| Ulid::from_str(s))
        .collect::<Vec<_>>();
    let mut uuids = ulids
        .iter()
        .map(|uuid| Uuid::from(*uuid).to_string())
        .collect::<Vec<_>>();
    uuids.sort();
    uuids.reverse();
    Ok(Json(uuids))
}

#[derive(Serialize)]
struct UlidsWeekdayResult {
    #[serde(rename = "christmas eve")]
    christmas_eve: usize,
    weekday: usize,
    #[serde(rename = "in the future")]
    in_the_future: usize,
    #[serde(rename = "LSB is 1")]
    lsb_is_1: usize,
}

async fn ulids_weekday(
    Path(weekday_req): Path<String>,
    Json(ulids): Json<UlidRequest>,
) -> Result<Json<UlidsWeekdayResult>, AppError> {
    let ulids = ulids
        .iter()
        .flat_map(|s| Ulid::from_str(s))
        .collect::<Vec<_>>();
    let mut result = UlidsWeekdayResult {
        christmas_eve: 0,
        weekday: 0,
        in_the_future: 0,
        lsb_is_1: 0,
    };
    ulids.iter().for_each(|ulid| {
        let datetime: DateTime<Utc> = ulid.datetime().into();
        let day = datetime.format("%d").to_string();
        let month = datetime.format("%m").to_string();
        let weekday = datetime.format("%w").to_string();
        if day == "24".to_string() && month == "12".to_string() {
            result.christmas_eve += 1;
        }
        let mut weekday_req = weekday_req.parse::<usize>().unwrap();
        weekday_req += 1;
        if weekday_req == 7 {
            weekday_req = 0;
        }
        if weekday == weekday_req.to_string() {
            result.weekday += 1;
        }
        if ulid.datetime() > SystemTime::now() {
            result.in_the_future += 1;
        }
        if ulid.0 & 1 == 1 {
            result.lsb_is_1 += 1;
        }
    });
    Ok(Json(result))
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/12/save/:packet_id", post(save_packet))
        .route("/12/load/:packet_id", get(load_packet))
        .route("/12/ulids", post(ulids))
        .route("/12/ulids/:weekday", post(ulids_weekday))
        .with_state(Arc::new(Mutex::new(AppState {
            multi_stopwatch: HashMap::new(),
        })))
}
