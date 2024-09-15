use axum::{http::StatusCode, routing::get, Router};
use sqlx::PgPool;

mod day1;
mod day11;
mod day12;
mod day13;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
pub mod error;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn fake_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[derive(Clone)]
struct CommonState {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(local_uri = "postgres://postgres:password@localhost:5432/cch23")]
    pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migration should run");

    let state = CommonState { pool };

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_error))
        .merge(day1::get_routes())
        .merge(day4::get_routes())
        .merge(day5::get_routes())
        .merge(day6::get_routes())
        .merge(day7::get_routes())
        .merge(day8::get_routes())
        .merge(day11::get_routes())
        .merge(day12::get_routes())
        .merge(day13::get_routes(state));

    Ok(router.into())
}
