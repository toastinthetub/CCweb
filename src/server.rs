use crate::user::{Language, User};
use axum::response::Html;

pub async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
