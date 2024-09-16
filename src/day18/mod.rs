use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    day13::{orders, reset},
    CommonState,
};

#[derive(Deserialize)]
struct Region {
    id: i64,
    name: String,
}

async fn add_region(region: Region, pool: &PgPool) {
    sqlx::query!(
        "INSERT INTO regions (id, name) VALUES ($1, $2)",
        region.id as i32,
        region.name,
    )
    .execute(pool)
    .await
    .unwrap();
}

#[derive(Serialize, Debug)]
struct RegionsTotalResponse {
    region: String,
    total: i64,
}

async fn regions_total(State(state): State<CommonState>) -> Json<Vec<RegionsTotalResponse>> {
    let regions: Vec<RegionsTotalResponse> = sqlx::query_as!(
        RegionsTotalResponse,
        r#"
        SELECT r.name as "region!", SUM(o.quantity) as "total!" 
        FROM orders o 
        JOIN regions r ON o.region_id = r.id
        GROUP BY r.name
        ORDER BY r.name ASC
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .unwrap();

    Json(regions)
}

async fn regions(State(state): State<CommonState>, Json(regions): Json<Vec<Region>>) {
    for region in regions {
        add_region(region, &state.pool).await;
    }
}

async fn get_regions(pool: &PgPool) -> Vec<Region> {
    let regions: Vec<Region> = sqlx::query_as!(
        Region,
        r#"
        SELECT id as "id!", name as "name!"
        FROM regions
        ORDER BY name ASC
        "#,
    )
    .fetch_all(pool)
    .await
    .unwrap();

    regions
}

#[derive(Serialize)]
struct RegionTopList {
    region: String,
    top_gifts: Vec<String>,
}

async fn get_top_list_for_region(region_name: String, limit: i64, pool: &PgPool) -> Vec<String> {
    let top_list = sqlx::query_scalar::<_, String>(
        r#"
        SELECT o.gift_name as "gift_name!"
        FROM orders o
        JOIN regions r ON o.region_id = r.id
        WHERE r.name = $1
        GROUP BY r.name, o.gift_name
        ORDER BY SUM(o.quantity) DESC, o.gift_name ASC
        LIMIT $2
        "#,
    )
    .bind(region_name)
    .bind(limit)
    .fetch_all(pool)
    .await
    .unwrap();

    top_list
}

async fn top_list(
    State(state): State<CommonState>,
    Path(number): Path<i64>,
) -> Json<Vec<RegionTopList>> {
    let regions = get_regions(&state.pool).await;
    let mut top_lists = Vec::<RegionTopList>::new();
    for region in regions {
        top_lists.push(RegionTopList {
            region: region.name.clone(),
            top_gifts: get_top_list_for_region(region.name, number, &state.pool).await,
        })
    }
    Json(top_lists)
}

pub fn get_routes(state: CommonState) -> Router {
    Router::new()
        .route("/18/reset", post(reset))
        .route("/18/orders", post(orders))
        .route("/18/regions", post(regions))
        .route("/18/regions/total", get(regions_total))
        .route("/18/regions/top_list/:number", get(top_list))
        .with_state(state)
}
