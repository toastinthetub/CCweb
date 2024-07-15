use crate::user::{self, Language, User};
use axum::{extract::Path, response::Html, routing::get, Form, Json, Router};
use serde::{Deserialize, Serialize};
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

pub async fn get_handler(Path(param): Path<(String, String)>) -> Json<String> {
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
            // return Html(content.to_owned());
            return Json(content);
        }
    }

    let user: Option<User> =
        match User::lookup_user(FILEPATH, &params.unwrap().user.unwrap().clone()) {
            Ok(user) => Some(user),
            Err(e) => {
                println!("Error: {:?}", e);
                None
            }
        };

    // let html_content = match user {
    //     Some(user) => {
    //         println!("served user: {:?}", user);
    //         format!(r#"<h1>REQUESTED USER: {:?}</h1>"#, user)
    //     }
    //     None => {
    //         format!(r#"<h1>NO USER FOUND!</h1>"#)
    //     }
    // };
    // let html_content = format!(r#"<h1>{:?}</h1>"#, param);
    // Html(html_content)

    let json_content = match user {
        Some(user) => {
            println!("served user: {:?}", user);
            format!(r#"REQUESTED USER: {:?}"#, serde_json::to_string(&user))
        }
        None => {
            format!(r#"USER {} NOT FOUND!"#, param.1)
        }
    };

    return Json(json_content);
}

pub async fn create_post_handler(
    Path(param): Path<(String, String, String, String, String)>,
) -> Json<String> /*Result<Result<Html<String>, Json<String>>, Box<dyn std::error::Error>>*/ {
    let params = match PathParams::from_post_list(axum::extract::Path(param.clone())) {
        Ok(params) => params,
        Err(e) => {
            println!("{:?}, {:?}", e, param);
            // let html = format!("bad params: {:?}", param);
            let json = format!("bad params: {:?}", param);
            // return Html(html);
            return Json(json);
        }
    };
    println!("{:?}", params);

    let languages = match parse_languages(&params.languages.unwrap()) {
        Ok(languages) => languages,
        Err(_) => {
            // println!("{:?}", languages);
            vec![Language::BadLanguage]
        }
    };

    match authenticate(params.key.clone().unwrap()) {
        Ok(_) => {
            println!("key {} accepted", params.key.clone().unwrap());
        }
        Err(e) => {
            // let html = format!("<h1>Invalid key: {}</1>", params.key.clone().unwrap());
            let json = format!("Error: invalid key: {:?}", params.key.clone().unwrap());
            println!("{:}", e);
            // return Html(html);
            return Json(json);
        }
    }

    match params.mode {
        Some(mode) => match mode {
            CommandMode::Create => {
                let user = match user::User::create_user(
                    params.user.clone(),
                    Some(languages),
                    params.discordid,
                ) {
                    Ok(user) => {
                        match user.save_to_csv(FILEPATH) {
                            Ok(_) => {
                                println!("successfully created user {:?}: {:?}", params.user, user);
                                let html = format!(
                                    "<h1>Successfully created user {:?}: {:?}</h1>",
                                    params.user,
                                    serde_json::to_string(&user)
                                );
                                let json = format!(
                                    "Successfully created user: {:?}\n{:?}",
                                    params.user,
                                    serde_json::to_string(&user)
                                );

                                // return  Ok(Err(Json(json)));
                                // return Ok(Ok(Html(html)));
                                // return Html(html);
                                return Json(json);
                            }
                            Err(e) => {
                                println!("failed to create user {:?}: {:?}", params.user, e);
                                let html = format!(
                                    "<h1>Failed to create user {:?}: {:?}</h1>",
                                    params.user, e
                                );
                                let json =
                                    format!("Failed to create user {:?}: {:?}", params.user, e);

                                //return Ok(Err(Json(json)));
                                // return Ok(Ok(Html(html)));
                                // return Html(html);
                                return Json(json);
                            }
                        }
                    }
                    Err(e) => {
                        println!("failed to create user {:?}: {:?}", params.user, e);
                        let html =
                            format!("<h1>Failed to create user {:?}: {:?}</h1>", params.user, e);
                        let json = format!("Failed to create user {:?}: {:?}", params.user, e);

                        //return Ok(Err(Json(json)));
                        // return Ok(Ok(Html(html)));
                        // return Html(html);
                        return Json(json);
                    }
                };
            }
            CommandMode::Destroy => {
                let user = match user::User::lookup_user(FILEPATH, &params.user.clone().unwrap()) {
                    Ok(user) => {
                        match user::User::remove_user(FILEPATH, &user.username) {
                            Ok(_) => {
                                println!("Successfully deleted user {:?}", user);
                                let html = format!(
                                    "<h1>Successfully deleted user: {:?}</h1>",
                                    params.user
                                );
                                let json =
                                    format!("Deleted user:\n{:?}", serde_json::to_string(&user));
                                // return Ok(Err(Json(content)));
                                // return Ok(Ok(Html(html)));
                                // return Html(html);
                                return Json(json);
                            }
                            Err(e) => {
                                println!("error: failed to delete user {:?}", user);
                                let html = format!(
                                    "<h1>Failed to delete user {:?}: {e}</h1>",
                                    params.user
                                );
                                let json = format!("Failed to delete user {:?}, {e}", params.user);
                                // return Ok(Err(Json(content)));
                                // return Ok(Ok(Html(html)));
                                // return Html(html);
                                return Json(json);
                            }
                        }
                    }
                    Err(e) => {
                        let html = format!("<h1>Could not find user: {:?}</h1>", e);
                        let json = format!("Could not find user: {:?}", e);
                        // return Ok(Err(Json(json)))
                        // return Ok(Ok(Html(html)));
                        // return Html(html);
                        return Json(json);
                    }
                };
            }
            CommandMode::AppendLanguage => {
                let mut user =
                    match user::User::lookup_user(FILEPATH, &params.user.clone().unwrap()) {
                        Ok(mut user) => {
                            match user::User::add_language(&mut user, languages.clone(), FILEPATH) {
                                Ok(_) => {
                                    println!(
                                        "Successfully appended languages {:?} to user {:?}",
                                        languages, params.user
                                    );
                                    let html = format!(
                                    "<h1>Successfully appended languages {:?} to user: {:?}</h1>",
                                    languages, params.user
                                    );
                                    let json = format!(
                                        "Successfully appended languages {:?} to user: {:?}",
                                        languages,
                                        serde_json::to_string(&user)
                                    );
                                    // return Ok(Err(Json(content)));
                                    // return Ok(Ok(Html(html)));
                                    // return Html(html);
                                    return Json(json);
                                }
                                Err(e) => {
                                    let html = format!(
                                        "<h1>Failed to append languages {:?}: {:?}</h1>",
                                        languages, e
                                    );
                                    let json = format!(
                                        "Failed to append languages {:?}: {:?}",
                                        languages, e
                                    );
                                    // return Ok(Err(Json(json)))
                                    // return Ok(Ok(Html(html)));
                                    // return Html(html);
                                    return Json(json);
                                }
                            }
                        }
                        Err(e) => {
                            let html = format!("<h1>Could not find user: {:?}</h1>", e);
                            let json = format!("Could not find user: {:?}", e);
                            return Json(json);
                            // return Ok(Ok(Html(html)));
                            // return Html(html);
                        }
                    };
            }
            CommandMode::RemoveLanguage => {
                let mut user =
                    match user::User::lookup_user(FILEPATH, &params.user.clone().unwrap()) {
                        Ok(mut user) => {
                            match user::User::remove_language(
                                &mut user,
                                languages.clone(),
                                FILEPATH,
                            ) {
                                Ok(_) => {
                                    println!(
                                        "Successfully removed languages {:?} from user {:?}",
                                        languages, params.user
                                    );
                                    let html = format!(
                                    "<h1>Successfully removed languages {:?} from user: {:?}</h1>",
                                    languages, params.user
                                    );
                                    let json = format!(
                                        "Successfully removed languages {:?} from user: {:?}",
                                        languages,
                                        serde_json::to_string(&user)
                                    );
                                    // return Ok(Err(Json(content)));
                                    // return Ok(Ok(Html(html)));
                                    // return Html(html);
                                    return Json(json);
                                }
                                Err(e) => {
                                    let html = format!(
                                        "<h1>Failed to remove languages {:?}: {:?}</h1>",
                                        languages, e
                                    );
                                    let json = format!(
                                        "Failed to remove languages {:?}: {:?}",
                                        languages, e
                                    );
                                    // return Ok(Err(Json(json)))
                                    // return Ok(Ok(Html(html)));
                                    // return Html(html);
                                    return Json(json);
                                }
                            }
                        }
                        Err(e) => {
                            let html = format!("<h1>Could not find user: {:?}</h1>", e);
                            let json = format!("Could not find user: {:?}", e);
                            // return Ok(Err(Json(json)))
                            // return Ok(Ok(Html(html)));
                            // return Html(html);
                            return Json(json);
                        }
                    };
            }
        },
        None => {
            let html = format!("<h1>Invalid mode: {:?}</h1>", params.mode);
            let json = format!("Invalid mode: {:?}", params.mode);
            // return Ok(Err(Json(json)))
            // return Ok(Ok(Html(html)));
            // return Html(html);
            return Json(json);
        }
    }

    let json = format!("this is json.");
    let html = format!("<h1>this is html. you reached the bottom of the function.</h1>"); // you should never see this

    // Ok(Err(Json(json)))
    // Ok(Ok(Html(html)))
    // Html(html)
    return Json(json);
}

pub async fn delete_post_handler(Path(param): Path<(String, String, String)>) -> Json<String> {
    let placeholder: String = String::from("monkey!");

    let key = param.0;
    let user = param.1;

    match authenticate(key.clone()) {
        Ok(_) => {
            println!("key {} accepted", key);
        }
        Err(e) => {
            // let html = format!("<h1>Invalid key: {}</1>", params.key.clone().unwrap());
            let json = format!("Error: invalid key: {:?}", key);
            println!("{:}", e);
            // return Html(html);
            return Json(json);
        }
    }

    match user::User::lookup_user(FILEPATH, &user) {
        Ok(_) => {}
        Err(_) => {
            let json = format!("USER {} NOT FOUND!", user);
            return Json(json);
        }
    }

    match user::User::remove_user(FILEPATH, &user) {
        Ok(_) => {
            let json: String = format!("USER {} SUCCESSFULLY DELETED!", user);
            return Json(json);
        }
        Err(e) => {
            let json: String = format!("FAILED TO DELETE USER {}: {}", user, e);
            return Json(json);
        }
    }
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
        let mode: String = params.1;
        let mode = mode.parse()?;
        let key: String = params.0;
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

fn parse_languages(languages_str: &str) -> Result<Vec<Language>, &str> {
    let mut languages_strs = Vec::new();
    let mut languages = Vec::new();
    for language in languages_str.split('|') {
        if language != "" {
            languages_strs.push(language);
            println!("{}", language)
        }
    }
    for language in languages_strs {
        let language = match Language::from_str(language) {
            Ok(lang) => languages.push(lang),
            Err(_) => {
                println!("{:?}", language);
                return Err(language);
            }
        };
        println!("{:?}", language)
    }
    // for language in languages_strs {
    //     match Language::from_str(language) {
    //         Ok(lang) => languages.push(lang),
    //         Err(_) => {
    //             println!("{:?}", language);
    //             return Err(language);
    //         }
    //     }
    // }5
    Ok(languages)
}
