use std::collections::HashMap;

use anyhow::anyhow;
use axum::{debug_handler, http::HeaderMap, routing::get, Json, Router};
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

fn decode_secret(input: &str) -> Result<String, anyhow::Error> {
    let encoded = input.strip_prefix("recipe=").unwrap();
    let decoded = general_purpose::STANDARD.decode(encoded).unwrap();
    Ok(String::from_utf8(decoded)?)
}

async fn decode_recipe(headers: HeaderMap) -> Result<String, AppError> {
    let recipe = if let Some(header) = headers.get("Cookie") {
        let encoded = header.to_str()?;
        decode_secret(encoded)
    } else {
        return Err(AppError(anyhow!("Unable to decode recipe")));
    };

    Ok(recipe?)
}

#[derive(Deserialize)]
struct Recipe {
    recipe: HashMap<String, i64>,
    pantry: HashMap<String, i64>,
}

#[derive(Serialize)]
struct BakeResponse {
    cookies: i64,
    pantry: HashMap<String, i64>,
}

impl BakeResponse {
    pub fn bake(n_cookies: i64, recipe: &Recipe) -> Self {
        let mut remaining_pantry = recipe.pantry.clone();
        for (ingredient, quantity) in recipe.recipe.iter() {
            remaining_pantry
                .entry(ingredient.to_string())
                .and_modify(|amount| *amount -= quantity * n_cookies);
        }
        BakeResponse {
            cookies: n_cookies,
            pantry: remaining_pantry,
        }
    }
}

#[debug_handler]
async fn bake(headers: HeaderMap) -> Result<Json<BakeResponse>, AppError> {
    let recipe = decode_recipe(headers).await?;
    let recipe: Recipe = serde_json::from_str(recipe.as_str())?;
    let mut max_cookies = Vec::<i64>::new();
    for (ingredient, amount) in recipe.recipe.iter() {
        if *amount > 0 {
            max_cookies.push(recipe.pantry.get(ingredient).unwrap_or(&0) / amount);
        }
    }
    let max_cookies = max_cookies.iter().min().unwrap_or(&0);
    Ok(Json(BakeResponse::bake(*max_cookies, &recipe)))
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/7/decode", get(decode_recipe))
        .route("/7/bake", get(bake))
}
