use crate::user::{Language, User};
use axum::{extract::Path, response::Html, routing::get, Router};
use serde::Deserialize;
use std::{env, error::Error, fmt};
use tokio::net::lookup_host;

pub const FILEPATH: &str = "./users.csv";

#[derive(Deserialize, Debug)]
struct PathParams {
    key: Option<String>,
    user: Option<String>,
    languages: Option<String>,
    discordid: Option<String>,
}

#[derive(Debug)]
enum AuthError {
    InvalidApiKey,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AuthError::InvalidApiKey => write!(f, "Invalid API key"),
        }
    }
}

impl Error for AuthError {}

pub async fn get_handler(Path(param): Path<(String, String)>) -> Html<String> {
    println!("get handler was called");

    let params = PathParams::from_get_list(axum::extract::Path(param.clone()));

    match authenticate(params.key.clone().unwrap()) {
        Ok(_) => {
            println!("key {} accepted", params.key.clone().unwrap());
        }
        Err(e) => {
            let content = format!("<h1>Invalid key: {}</1>", params.key.clone().unwrap());
            println!("{:}", e);
            return Html(content.to_owned());
        }
    }

    let user: Option<User> = match User::lookup_user(FILEPATH, &params.user.unwrap()) {
        Ok(user) => Some(user),
        Err(e) => {
            println!("Error: {:?}", e);
            None
        }
    };

    let html_content = match user {
        Some(user) => {
            println!("server user: {:?}", user);
            format!(r#"<h1>REQUESTED USER: {:?}</h1>"#, user)
        }
        None => {
            format!(r#"<h1>NO USER FOUND!</h1>"#)
        }
    };
    // let html_content = format!(r#"<h1>{:?}</h1>"#, param);
    Html(html_content)
}

pub async fn post_handler(Path(param): Path<(String, String, String, String)>) -> Html<String> {
    let params = PathParams::from_post_list(axum::extract::Path(param.clone()));

    match authenticate(params.key.clone().unwrap()) {
        Ok(_) => {
            println!("key {} accepted", params.key.clone().unwrap());
        }
        Err(e) => {
            let content = format!("<h1>Invalid key: {}</1>", params.key.clone().unwrap());
            println!("{:}", e);
            return Html(content.to_owned());
        }
    }

    let html_content = format!(r#"<h1>{:?}</h1>"#, param);
    Html(html_content)
}

fn authenticate(key: String) -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let api_key = env::var("API_KEY").expect("API_KEY not set in .env file");

    if key == api_key {
        Ok(())
    } else {
        Err(Box::new(AuthError::InvalidApiKey))
    }
}

impl PathParams {
    fn from_get_list(Path(params): Path<(String, String)>) -> Self {
        let key: String = params.0;
        let user: String = params.1;
        Self {
            key: Some(key),
            user: Some(user),
            languages: None,
            discordid: None,
        }
    }
    fn from_post_list(Path(params): Path<(String, String, String, String)>) -> Self {
        let key: String = params.0;
        let user: String = params.1;
        let languages: String = params.2;
        let discordid: String = params.3;
        Self {
            key: Some(key),
            user: Some(user),
            languages: Some(languages),
            discordid: Some(discordid),
        }
    }
}
