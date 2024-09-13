use axum::{http::StatusCode, routing::get, Router};

mod day1;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn fake_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_error))
        .merge(day1::get_routes())
        .merge(day4::get_routes())
        .merge(day5::get_routes())
        .merge(day6::get_routes())
        .merge(day7::get_routes())
        .merge(day8::get_routes());

    Ok(router.into())
}
