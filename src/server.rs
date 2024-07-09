use crate::user::{self, Language, User};
use axum::{extract::Path, response::Html, routing::get, Router};
use serde::Deserialize;
use std::{env, error::Error, fmt, str::FromStr, vec};
use tokio::net::lookup_host;

pub const FILEPATH: &str = "./users.csv";

#[derive(Deserialize, Debug)]
struct PathParams {
    mode: Option<CommandMode>,
    key: Option<String>,
    user: Option<String>,
    languages: Option<String>,
    discordid: Option<String>,
}

#[derive(Deserialize, Debug)]
enum CommandMode {
    Create,
    Destroy,
    AppendLanguage,
    RemoveLanguage,
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

    match authenticate(params.as_ref().unwrap().key.clone().unwrap()) {
        Ok(_) => {
            println!(
                "key {} accepted",
                params.as_ref().unwrap().key.clone().unwrap()
            );
        }
        Err(e) => {
            let content = format!(
                "<h1>Invalid key: {}</1>",
                params.unwrap().key.clone().unwrap()
            );
            println!("{:}", e);
            return Html(content.to_owned());
        }
    }

    let user: Option<User> = match User::lookup_user(FILEPATH, &params.unwrap().user.unwrap()) {
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

pub async fn post_handler(
    Path(param): Path<(String, String, String, String, String)>,
) -> Html<String> {
    let params = PathParams::from_post_list(axum::extract::Path(param.clone())).unwrap();

    let languages = match parse_languages(&params.languages.unwrap()) {
        Ok(languages) => languages,
        Err(_) => {
            vec![Language::C]
        }
    };

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

    match params.mode {
        Some(mode) => match mode {
            CommandMode::Create => {
                user::User::create_user(params.user, Some(languages), params.discordid)
            }
            CommandMode::Destroy => user::User::remove_user(FILEPATH, params.user),
            CommandMode::AppendLanguage => todo!(),
            CommandMode::RemoveLanguage => todo!(),
        },
        None => todo!(),
    }
    .unwrap();

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
    fn from_get_list(
        Path(params): Path<(String, String)>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let key: String = params.0;
        let user: String = params.1;
        Ok(Self {
            mode: None,
            key: Some(key),
            user: Some(user),
            languages: None,
            discordid: None,
        })
    }
    fn from_post_list(
        Path(params): Path<(String, String, String, String, String)>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mode: String = params.0;
        let mode = mode.parse()?;
        let key: String = params.1;
        let user: String = params.2;
        let languages: String = params.3;
        let discordid: String = params.4;
        Ok(Self {
            mode: Some(mode),
            key: Some(key),
            user: Some(user),
            languages: Some(languages),
            discordid: Some(discordid),
        })
    }
}

impl FromStr for CommandMode {
    type Err = ParseCommandModeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "c" => Ok(CommandMode::Create),
            "d" => Ok(CommandMode::Destroy),
            "a" => Ok(CommandMode::AppendLanguage),
            "r" => Ok(CommandMode::RemoveLanguage),
            _ => Err(ParseCommandModeError),
        }
    }
}

#[derive(Debug)]
struct ParseCommandModeError;

impl fmt::Display for ParseCommandModeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid command mode")
    }
}

impl std::error::Error for ParseCommandModeError {}

fn parse_languages(languages_str: &str) -> Result<Vec<Language>, ()> {
    let mut languages = Vec::new();
    for language in languages_str.split('|') {
        match Language::from_str(language) {
            Ok(lang) => languages.push(lang),
            Err(_) => return Err(()),
        }
    }
    Ok(languages)
}
