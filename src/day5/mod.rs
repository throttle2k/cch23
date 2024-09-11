use axum::{extract::Query, routing::post, Json, Router};
use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    offset: Option<usize>,
    limit: Option<usize>,
    split: Option<usize>,
}

async fn paginate_list(pagination: Query<Pagination>, Json(names): Json<Vec<String>>) -> String {
    let offset = match pagination.offset {
        Some(o) => o,
        None => 0,
    };
    let limit = match pagination.limit {
        Some(l) => l,
        None => names.len(),
    };
    let result = match pagination.split {
        Some(split) => {
            let v = names
                .iter()
                .skip(offset)
                .take(limit)
                .cloned()
                .collect::<Vec<String>>()
                .chunks(split)
                .map(|c| c.to_vec())
                .collect::<Vec<Vec<String>>>();
            format!("{:?}", v)
        }
        None => {
            let v = names
                .iter()
                .skip(offset)
                .take(limit)
                .cloned()
                .collect::<Vec<String>>();
            format!("{:?}", v)
        }
    };
    result
}

pub fn get_routes() -> Router {
    Router::new().route("/5", post(paginate_list))
}
