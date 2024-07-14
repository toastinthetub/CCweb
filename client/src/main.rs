use crossterm::{
    cursor::{self, position, MoveTo},
    queue,
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
    let mut input = String::new();
    let mode: Mode;

    let mut stdout = std::io::stdout();

    loop {
        print!("{}", SHELL);
        stdout.flush().unwrap();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read for some reason");
        match input.trim_end() {
            "get" => {
                mode = Mode::Get;
                break;
            }
            "post" => {
                mode = Mode::Post;
                break;
            }
            "help" => {
                todo!();
                // help();
                // TODO: Help
                break;
            }
            _ => {
                println!("{}", input);
                println!("Command: '{}' not understood. try pass 'help'", input);
            }
        }
        input.clear();
    }
    match mode {
        Mode::Get => {
            let cc_request = GetRequest::build_get_request(api_key);
            println!("{}sending GET request to '{}'", SHELL, cc_request.string());
            let task = task::spawn(async move { cc_request.make_get_request(client).await });

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
                    string.push_str(".");
                }

                print!("{}", string);
                stdout.flush().unwrap();

                counter += 1;
            }

            let result = task.await.unwrap();
            println!("");
            println!("{}task completed, result:", SHELL);
            println!("{}", result);
        }
        Mode::Post => {}
    }
}

impl GetRequest {
    fn build_get_request(key: String) -> Self {
        // this has to own it cause make_get_request() is async
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
        let url = format!("{}/{}/{}/", self.base, self.key, self.user);
        url
    }
}

impl PostRequest {
    fn build_post_request(key: String) -> () /* Self */ {
        let mut stdout = std::io::stdout();

        // key

        let mut mode: String = String::new();
        let mut user: String = String::new();
        let mut languages_str = String::new();
        let mut languages: Vec<String> = Vec::new();
        let mut discordid: String = String::new();

        print!("{}mode (c(reate), d(estroy), a(ppend), r(emove)): ", SHELL);
        stdout.flush().unwrap();
        std::io::stdin()
            .read_line(&mut mode)
            .expect("failed to read for some reason");

        let mode = mode.trim_end();

        print!("{}languages (seperate by commas, or leave blank): ", SHELL);
        stdout.flush().unwrap();
        std::io::stdin()
            .read_line(&mut languages_str)
            .expect("failed to read for some reason");

        let languages_str = languages_str.trim_end();

        print!("{}discord ID (leave blank for none): ", SHELL);
        stdout.flush().unwrap();
        std::io::stdin()
            .read_line(&mut discordid)
            .expect("kill yourself");

        let languages_str: String = languages_str
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();

        let tmp: Vec<&str> = languages_str.split(',').collect();

        for string in tmp {
            let mut string = string.to_uppercase();
            string.insert(0, '|');
            languages.push(string);
        }

        // Self {}
        ()
    }
}
