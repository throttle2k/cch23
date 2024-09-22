use std::fmt::Display;

use anyhow::Result;
use axum::{extract::Path, routing::get, Router};
use country_boundaries::{CountryBoundaries, LatLon, BOUNDARIES_ODBL_360X180};
use dms_coordinates::DMS;
use s2::{cell, cellid::CellID};

use crate::error::AppError;

struct DdegCoords {
    ddeg_latitude: f64,
    ddeg_longitude: f64,
}

struct DmsCoords {
    dms_latitude: DMS,
    dms_longitude: DMS,
}

impl From<DdegCoords> for DmsCoords {
    fn from(coords: DdegCoords) -> Self {
        let dms_latitude = DMS::from_ddeg_latitude(coords.ddeg_latitude);
        let dms_longitude = DMS::from_ddeg_longitude(coords.ddeg_longitude);
        Self {
            dms_latitude,
            dms_longitude,
        }
    }
}

fn format_dms(dms: DMS) -> String {
    format!(
        "{}°{}'{:.3}''{}",
        dms.degrees,
        dms.minutes,
        dms.seconds,
        dms.cardinal.unwrap()
    )
}

impl Display for DmsCoords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            format_dms(self.dms_latitude),
            format_dms(self.dms_longitude)
        )
    }
}

fn get_coordinates(binary: &str) -> Result<DdegCoords> {
    let n = u64::from_str_radix(binary, 2)?;
    let cell_id = CellID(n);
    let point = cell::Cell::from(cell_id).center();
    let latitude = point.latitude().deg();
    let longitude = point.longitude().deg();
    Ok(DdegCoords {
        ddeg_latitude: latitude,
        ddeg_longitude: longitude,
    })
}

async fn coords(Path(binary): Path<String>) -> Result<String, AppError> {
    let ddeg_coords = get_coordinates(&binary)?;
    let dms_coords = DmsCoords::from(ddeg_coords);
    Ok(format!("{}", dms_coords))
}

async fn country(Path(binary): Path<String>) -> Result<String, AppError> {
    let ddeg_coords = get_coordinates(&binary)?;
    let lat_lon = LatLon::new(ddeg_coords.ddeg_latitude, ddeg_coords.ddeg_longitude)?;
    let boundaries = CountryBoundaries::from_reader(BOUNDARIES_ODBL_360X180)?;
    let country_name = rust_iso3166::from_alpha2(boundaries.ids(lat_lon).last().unwrap())
        .unwrap()
        .name;
    Ok(country_name.split(' ').next().unwrap().to_string())
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/21/coords/:binary", get(coords))
        .route("/21/country/:binary", get(country))
}
