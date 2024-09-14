use axum::{extract::Path, routing::get, Router};

async fn recalibrate_packet_ids(Path(packets): Path<String>) -> String {
    let splits = packets.split('/');
    let mut ids = Vec::<i32>::new();
    for split in splits {
        if let Ok(v) = split.parse::<i32>() {
            ids.push(v);
        }
    }
    let result = ids.iter().fold(0, |acc, v| acc ^ v).pow(3);
    result.to_string()
}

pub fn get_routes() -> Router {
    Router::new().route("/1/*packets", get(recalibrate_packet_ids))
}
