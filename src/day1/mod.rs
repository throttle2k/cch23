use axum::{extract::Path, routing::get, Router};

async fn recalibrate_packet_ids(Path(packets): Path<String>) -> String {
    packets
        .split('/')
        .map(|e| e.parse::<i32>().unwrap())
        .fold(0, |acc, v| acc ^ v)
        .pow(3)
        .to_string()
}

pub fn get_routes() -> Router {
    Router::new().route("/1/*packets", get(recalibrate_packet_ids))
}
