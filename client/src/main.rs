use crossterm::{
    cursor::{self, position, MoveTo},
    queue,
    style::Stylize,
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};

use reqwest::Client;
use std::{env, io::Write};
use tokio::task;

const BASE: &str = "http://172.233.158.174:3000";
const SHELL: &str = "[CCWC] > ";

#[derive(Clone)]
enum Mode {
    Get,
    Post,
}

#[derive(Clone)]
struct GetRequest {
    base: String,
    key: String,
    user: String,
}

#[derive(Clone)]
struct PostRequest {
    base: String,
    key: String,
    mode: String,
    languages: Option<Vec<String>>,
    user: String,
    discord_id: Option<String>,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY not set in .env file");

    let client = Client::new();
    let mut stdout = std::io::stdout();

    loop {
        let mode = get_user_mode(&mut stdout);

        match mode {
            Mode::Get => {
                let cc_request = GetRequest::build_get_request(api_key.clone());
                println!("{}sending GET request to '{}'", SHELL, cc_request.string());
                let client_clone = client.clone();
                let task =
                    task::spawn(async move { cc_request.make_get_request(client_clone).await });

                display_loading(&mut stdout, task).await;
            }
            Mode::Post => {
                let mut cc_post = PostRequest::build_post_request(api_key.clone());
                println!("{}sending POST request to '{}'", SHELL, cc_post.string());
                let client_clone = client.clone();
                let task =
                    task::spawn(async move { cc_post.make_post_request(client_clone).await });

                display_loading(&mut stdout, task).await;
            }
        }

        println!("Do you want to make another request? (y/n)");
        stdout.flush().unwrap();
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read for some reason");
        if input.trim_end().to_lowercase() != "y" {
            break;
        }
    }
}

fn get_user_mode(stdout: &mut std::io::Stdout) -> Mode {
    let mut input = String::new();
    loop {
        print!("{}", SHELL);
        stdout.flush().unwrap();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read for some reason");
        match input.trim_end() {
            "get" => return Mode::Get,
            "post" => return Mode::Post,
            "help" => {
                todo!();
            }
            _ => {
                println!(
                    "Command: '{}' not understood. try pass 'help'",
                    input.trim_end()
                );
            }
        }
        input.clear();
    }
}

async fn display_loading<T>(stdout: &mut std::io::Stdout, task: task::JoinHandle<T>) -> T {
    let (_, y) = crossterm::cursor::position().unwrap();
    let mut counter: u16 = 0;

    while !task.is_finished() {
        queue!(stdout, Clear(ClearType::CurrentLine), MoveTo(0, y)).unwrap();
        stdout.flush().unwrap();

        if counter > 10 {
            counter = 0;
        }

        let mut string = String::new();
        for _ in 0..counter {
            string.push('.');
        }

        print!("{}", string);
        stdout.flush().unwrap();

        counter += 1;
    }

    println!("");
    println!("{}task completed, result:", SHELL);
    task.await.unwrap()
}

impl GetRequest {
    fn build_get_request(key: String) -> Self {
        let mut stdout = std::io::stdout();
        let mut user: String = String::new();
        print!("{}enter a username to lookup: ", SHELL);
        stdout.flush().unwrap();
        std::io::stdin()
            .read_line(&mut user)
            .expect("failed to read for some reason");
        Self {
            base: BASE.to_owned(),
            key: key.to_owned(),
            user: user.trim_end().to_string(),
        }
    }

    async fn make_get_request(&self, client: Client) -> String {
        let url = format!("{}/{}/{}/", self.base, self.key, self.user);
        let response = client.get(url).send().await.unwrap().text().await.unwrap();
        response
    }

    fn string(&self) -> String {
        format!("{}/{}/{}/", self.base, self.key, self.user)
    }
}

impl PostRequest {
    fn build_post_request(key: String) -> Self {
        let mut stdout = std::io::stdout();
        let mut mode = String::new();
        let mut user = String::new();
        let mut languages_str = String::new();
        let mut languages = Some(Vec::new());
        let mut discordid = Some(String::new());

        loop {
            print!("{}mode (c(reate), d(estroy), a(ppend), r(emove)): ", SHELL);
            stdout.flush().unwrap();
            std::io::stdin()
                .read_line(&mut mode)
                .expect("failed to read for some reason");

            if ["c", "d", "a", "r"].contains(&mode.trim_end()) {
                break;
            } else {
                println!("invalid mode! must be one of [c(reat), d(estroy), a(ppend), r(emove)]");
                mode.clear();
            }
        }

        print!("{}username: ", SHELL);
        stdout.flush().unwrap();
        std::io::stdin()
            .read_line(&mut user)
            .expect("failed to read for some reason");

        print!("{}languages (separate by commas, or leave blank): ", SHELL);
        stdout.flush().unwrap();
        std::io::stdin()
            .read_line(&mut languages_str)
            .expect("failed to read for some reason");

        print!("{}discord ID (leave blank for none): ", SHELL);
        stdout.flush().unwrap();
        std::io::stdin()
            .read_line(&mut discordid.as_mut().unwrap())
            .expect("failed to read for some reason");

        if discordid.as_ref().unwrap().is_empty() {
            discordid = None;
        }

        if !languages_str.trim_end().is_empty() {
            let languages_vec: Vec<String> = languages_str
                .trim_end()
                .split(',')
                .map(|s| format!("|{}", s.trim().to_uppercase()))
                .collect();
            languages = Some(languages_vec);
        } else {
            languages = None;
        }

        Self {
            base: BASE.to_owned(),
            key,
            mode: mode.trim_end().to_owned(),
            user: user.trim_end().to_owned(),
            languages,
            discord_id: discordid,
        }
    }

    async fn make_post_request(&mut self, client: Client) -> String {
        let languages_str = if let Some(languages) = &self.languages {
            languages.join("")
        } else {
            "null".to_owned()
        };

        let discord_id_str = if let Some(discord_id) = &self.discord_id {
            discord_id.clone()
        } else {
            "null".to_owned()
        };

        let url = format!(
            "{}/{}/{}/{}/{}/{}",
            self.base, self.key, self.mode, self.user, languages_str, discord_id_str
        );

        let response = client
            .post(&url)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        response
    }

    fn string(&self) -> String {
        let languages_str = if let Some(languages) = &self.languages {
            languages.join("")
        } else {
            "null".to_owned()
        };

        let discord_id_str = if let Some(discord_id) = &self.discord_id {
            discord_id.clone()
        } else {
            "null".to_owned()
        };

        format!(
            "{}/{}/{}/{}/{}/{}",
            self.base, self.key, self.mode, self.user, languages_str, discord_id_str
        )
    }
}
