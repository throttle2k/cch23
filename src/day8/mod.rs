use axum::{extract::Path, response::IntoResponse, routing::get, Router};
use reqwest::StatusCode;
use serde::Deserialize;

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[derive(Deserialize)]
struct PokemonData {
    // id: usize,
    // name: String,
    weight: i32,
}

async fn weight(Path(pokedex_humber): Path<usize>) -> Result<String, AppError> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokedex_humber);
    let body = reqwest::get(url).await?.text().await?;
    let data: PokemonData = serde_json::from_str(&body)?;
    let weight_in_kilos = data.weight as f64 / 10.0;
    Ok(weight_in_kilos.to_string())
}

async fn drop(Path(pokedex_humber): Path<usize>) -> Result<String, AppError> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokedex_humber);
    let body = reqwest::get(url).await?.text().await?;
    let data: PokemonData = serde_json::from_str(&body)?;
    let weight_in_kilos = data.weight as f64 / 10.0;
    let v = f64::sqrt(2.0 * 9.825 * 10.0);
    let momentum = v * weight_in_kilos;
    Ok(momentum.to_string())
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/8/weight/:pokedex_number", get(weight))
        .route("/8/drop/:pokedex_number", get(drop))
}
