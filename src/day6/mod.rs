use axum::{routing::post, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct CountElvesResponse {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf_on_it: usize,
}

fn count_occurrences(needle: &str, haystack: &str) -> usize {
    let mut count = 0;
    if haystack.len() > needle.len() {
        for i in 0..=(haystack.len() - needle.len()) {
            if haystack[i..].starts_with(needle) {
                count += 1;
            }
        }
    }
    count
}

impl From<&str> for CountElvesResponse {
    fn from(value: &str) -> Self {
        let elf = count_occurrences("elf", value);
        let elf_on_a_shelf = count_occurrences("elf on a shelf", value);
        let shelf = count_occurrences("shelf", value);
        let shelf_with_no_elf_on_it = shelf - elf_on_a_shelf;
        CountElvesResponse {
            elf,
            elf_on_a_shelf,
            shelf_with_no_elf_on_it,
        }
    }
}

async fn count_elves(text: String) -> Json<CountElvesResponse> {
    Json(CountElvesResponse::from(text.as_str()))
}

pub fn get_routes() -> Router {
    Router::new().route("/6", post(count_elves))
}
