use std::io::Cursor;

use axum::{
    async_trait,
    body::Bytes,
    extract::{FromRequest, Multipart, Request},
    http::{header::CONTENT_TYPE, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};
use image::{GenericImageView, ImageReader};
use tower_http::services::ServeDir;

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

struct PngImage(Bytes);

#[async_trait]
impl<S> FromRequest<S> for PngImage
where
    Bytes: FromRequest<S>,
    S: Send + Sync,
{
    type Rejection = AppError;
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Some(content_type) = req.headers().get(CONTENT_TYPE) else {
            return Err(AppError(anyhow::anyhow!("Missing content-type header")));
        };

        let body = if content_type.to_str()?.starts_with("multipart/form-data") {
            let mut multipart = Multipart::from_request(req, state)
                .await
                .map_err(|err| AppError(anyhow::Error::new(err)))?;

            let Ok(Some(field)) = multipart.next_field().await else {
                return Err(AppError(anyhow::anyhow!("Missing multipart field")));
            };

            field
                .bytes()
                .await
                .map_err(|err| AppError(anyhow::Error::new(err)))?
        } else {
            return Err(AppError(anyhow::anyhow!("Bad request")));
        };
        Ok(Self(body))
    }
}

async fn red_pixels(PngImage(image): PngImage) -> Result<String, AppError> {
    let img = ImageReader::new(Cursor::new(image))
        .with_guessed_format()?
        .decode()?;
    let mut count = 0;
    for (_, _, rgba) in img.pixels() {
        let colors = rgba.0;
        let r = colors[0] as u32;
        let g = colors[1] as u32;
        let b = colors[2] as u32;
        if r > g + b {
            count += 1;
        }
    }
    Ok(count.to_string())
}

pub fn get_routes() -> Router {
    Router::new()
        .nest_service("/11/assets", ServeDir::new("resources"))
        .route("/11/red_pixels", post(red_pixels))
}
