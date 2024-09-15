use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::CommonState;

async fn reset(State(state): State<CommonState>) -> StatusCode {
    sqlx::query!(
        r#"
TRUNCATE TABLE orders
"#
    )
    .execute(&state.pool)
    .await
    .unwrap();
    StatusCode::OK
}

async fn sql(State(state): State<CommonState>) -> String {
    let record = sqlx::query!(
        r#"
SELECT 20231213 as "id!"
"#
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();

    record.id.to_string()
}

#[derive(Deserialize)]
struct Order {
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

async fn add_order(order: Order, pool: &PgPool) {
    sqlx::query!(
        "INSERT INTO orders (id, region_id, gift_name, quantity) VALUES ($1, $2, $3, $4)",
        order.id,
        order.region_id,
        order.gift_name,
        order.quantity
    )
    .execute(pool)
    .await
    .unwrap();
}

async fn orders(State(state): State<CommonState>, Json(orders): Json<Vec<Order>>) -> StatusCode {
    for order in orders {
        add_order(order, &state.pool).await;
    }
    StatusCode::OK
}

#[derive(Serialize)]
struct OrderTotalResponse {
    total: i64,
}

async fn orders_total(State(state): State<CommonState>) -> Json<OrderTotalResponse> {
    if let Ok(result) = sqlx::query!(r#"SELECT SUM(quantity) as "total!" FROM orders"#)
        .fetch_one(&state.pool)
        .await
    {
        Json(OrderTotalResponse {
            total: result.total,
        })
    } else {
        Json(OrderTotalResponse { total: 0 })
    }
}

#[derive(Serialize)]
struct OrderPopularResponse {
    popular: Option<String>,
}

async fn orders_popular(State(state): State<CommonState>) -> Json<OrderPopularResponse> {
    if let Ok(result) = sqlx::query!(
        r#"
        SELECT gift_name, SUM(quantity) as total
        FROM orders
        GROUP BY gift_name
        ORDER BY total DESC
        "#
    )
    .fetch_one(&state.pool)
    .await
    {
        Json(OrderPopularResponse {
            popular: result.gift_name,
        })
    } else {
        Json(OrderPopularResponse { popular: None })
    }
}

pub fn get_routes<S>(state: CommonState) -> Router<S> {
    Router::new()
        .route("/13/sql", get(sql))
        .route("/13/reset", post(reset))
        .route("/13/orders", post(orders))
        .route("/13/orders/total", get(orders_total))
        .route("/13/orders/popular", get(orders_popular))
        .with_state(state)
}
