use axum::{http::StatusCode, routing::post, Json, Router};
use fancy_regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Deserialize)]
struct Password {
    input: String,
}

#[derive(Serialize)]
struct CheckNiceResult {
    result: String,
}

#[derive(Serialize)]
struct CheckGameResult {
    result: String,
    reason: String,
}

fn contains_three_vowels(input: &str) -> bool {
    let vowels = input
        .chars()
        .filter(|c| match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        })
        .count();
    vowels >= 3
}

fn one_letter_twice(input: &str) -> bool {
    let chars = input.chars();
    let mut peekable = chars.peekable();

    let mut is_valid = false;

    while let Some(curr) = peekable.next() {
        if let Some(next) = peekable.peek() {
            is_valid = curr.is_alphabetic() && curr == *next;
            if is_valid {
                break;
            };
        }
    }
    is_valid
}

fn contains_substrings(input: &str) -> bool {
    let contains_ab = input.contains("ab");
    let contains_cd = input.contains("cd");
    let contains_pq = input.contains("pq");
    let contains_xy = input.contains("xy");
    contains_ab || contains_cd || contains_pq || contains_xy
}

fn is_nice(input: &str) -> Result<(), String> {
    if !contains_three_vowels(input) {
        Err("Three vowels".to_string())
    } else if !one_letter_twice(input) {
        Err("One letter twice".to_string())
    } else if contains_substrings(input) {
        Err("Substring".to_string())
    } else {
        Ok(())
    }
}

async fn nice(Json(password): Json<Password>) -> (StatusCode, Json<CheckNiceResult>) {
    match is_nice(&password.input) {
        Ok(()) => (
            StatusCode::OK,
            Json(CheckNiceResult {
                result: "nice".to_string(),
            }),
        ),
        Err(_) => (
            StatusCode::BAD_REQUEST,
            Json(CheckNiceResult {
                result: "naughty".to_string(),
            }),
        ),
    }
}

fn check_rule_1(input: &str) -> bool {
    input.len() >= 8
}

fn check_rule_2(input: &str) -> bool {
    let contains_lowercase = input.chars().any(|c| matches!(c, 'a'..='z'));
    let contains_uppercase = input.chars().any(|c| matches!(c, 'A'..='Z'));
    let contains_digits = input.chars().any(|c| matches!(c, '0'..='9'));
    contains_lowercase && contains_uppercase && contains_digits
}

fn check_rule_3(input: &str) -> bool {
    let digits = input.chars().filter(|c| c.is_numeric()).count();
    digits >= 5
}

fn check_rule_4(input: &str) -> bool {
    let mut ints = Vec::<i32>::new();

    let mut current_int = String::new();
    for c in input.chars() {
        if c.is_numeric() {
            current_int.push(c);
        } else if current_int.len() > 0 {
            ints.push(current_int.parse().unwrap());
            current_int = String::new();
        }
    }
    let sum: i32 = ints.iter().sum();
    sum == 2023
}

fn check_rule_5(input: &str) -> bool {
    let joy = input
        .chars()
        .filter(|c| match c {
            'j' | 'o' | 'y' => true,
            _ => false,
        })
        .collect::<String>();
    joy == "joy".to_string()
}

fn check_rule_6(input: &str) -> bool {
    let rule = Regex::new(r"([a-zA-Z])\w\1").unwrap();
    rule.is_match(input).unwrap()
}

fn check_rule_7(input: &str) -> bool {
    let rule = Regex::new(r"[\u{2980}-\u{2BFF}]").unwrap();
    rule.is_match(input).unwrap()
}

fn check_rule_8(input: &str) -> bool {
    emojito::find_emoji(input).len() > 0
}

fn check_rule_9(input: &str) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let hash = hasher.finalize();
    hex::encode(hash).ends_with('a')
}

fn is_valid(input: &str) -> Result<(), (usize, String)> {
    if !check_rule_1(input) {
        Err((1, "8 chars".to_string()))
    } else if !check_rule_2(input) {
        Err((2, "more types of chars".to_string()))
    } else if !check_rule_3(input) {
        Err((3, "55555".to_string()))
    } else if !check_rule_4(input) {
        Err((4, "math is hard".to_string()))
    } else if !check_rule_5(input) {
        Err((5, "not joyful enough".to_string()))
    } else if !check_rule_6(input) {
        Err((6, "illegal: no sandwich".to_string()))
    } else if !check_rule_7(input) {
        Err((7, "outranged".to_string()))
    } else if !check_rule_8(input) {
        Err((8, "😳".to_string()))
    } else if !check_rule_9(input) {
        Err((9, "not a coffee brewer".to_string()))
    } else {
        Ok(())
    }
}

async fn game(Json(password): Json<Password>) -> (StatusCode, Json<CheckGameResult>) {
    match is_valid(&password.input) {
        Ok(()) => (
            StatusCode::OK,
            Json(CheckGameResult {
                result: "nice".to_string(),
                reason: "that's a nice password".to_string(),
            }),
        ),
        Err((1 | 2 | 3 | 4, s)) => (
            StatusCode::BAD_REQUEST,
            Json(CheckGameResult {
                result: "naughty".to_string(),
                reason: s,
            }),
        ),
        Err((5, s)) => (
            StatusCode::NOT_ACCEPTABLE,
            Json(CheckGameResult {
                result: "naughty".to_string(),
                reason: s,
            }),
        ),
        Err((6, s)) => (
            StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
            Json(CheckGameResult {
                result: "naughty".to_string(),
                reason: s,
            }),
        ),
        Err((7, s)) => (
            StatusCode::RANGE_NOT_SATISFIABLE,
            Json(CheckGameResult {
                result: "naughty".to_string(),
                reason: s,
            }),
        ),
        Err((8, s)) => (
            StatusCode::UPGRADE_REQUIRED,
            Json(CheckGameResult {
                result: "naughty".to_string(),
                reason: s,
            }),
        ),
        Err((9, s)) => (
            StatusCode::IM_A_TEAPOT,
            Json(CheckGameResult {
                result: "naughty".to_string(),
                reason: s,
            }),
        ),
        Err(_) => (
            StatusCode::BAD_REQUEST,
            Json(CheckGameResult {
                result: "naughty".to_string(),
                reason: "uncaught error".to_string(),
            }),
        ),
    }
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/15/nice", post(nice))
        .route("/15/game", post(game))
}
