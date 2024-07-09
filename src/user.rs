use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    username: String,
    languages: Vec<Language>,
    discord_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Language {
    C,
    CPP,
    Lua,
    Rust,
    Java,
    Kotlin,
    JavaScript,
    TypeScript,
}

#[derive(Debug)]
pub enum DatabaseError {
    MissingUsername,
    UserNotFound,
    UserAlreadyExists,
    IoError(io::Error),
}

impl From<io::Error> for DatabaseError {
    fn from(error: io::Error) -> Self {
        DatabaseError::IoError(error)
    }
}

impl FromStr for Language {
    type Err = ();

    fn from_str(input: &str) -> Result<Language, Self::Err> {
        match input {
            "C" => Ok(Language::C),
            "CPP" => Ok(Language::CPP),
            "Rust" => Ok(Language::Rust),
            "Java" => Ok(Language::Java),
            "Kotlin" => Ok(Language::Kotlin),
            "JavaScript" => Ok(Language::JavaScript),
            "TypeScript" => Ok(Language::TypeScript),
            _ => Err(()),
        }
    }
}

impl User {
    pub fn create_user(
        username: Option<String>,
        languages: Option<Vec<Language>>,
        discord_id: Option<String>,
    ) -> Result<Self, DatabaseError> {
        let username = match username {
            Some(u) => u,
            None => return Err(DatabaseError::MissingUsername),
        };
        let languages = languages.unwrap_or_default();
        let discord_id = discord_id.unwrap_or_default();

        Ok(User {
            username,
            languages,
            discord_id,
        })
    }

    pub fn add_language(&mut self, language: Language) {
        if !self.languages.contains(&language) {
            self.languages.push(language);
        }
    }

    pub fn remove_language(&mut self, language: &Language) {
        self.languages.retain(|l| l != language);
    }

    pub fn lookup_user(file_path: &str, username: &str) -> Result<User, DatabaseError> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 3 && parts[0] == username {
                let username = parts[0].to_string();
                let languages = parts[1]
                    .split('|')
                    .filter_map(|s| match s {
                        "C" => Some(Language::C),
                        "CPP" => Some(Language::CPP),
                        "Lua" => Some(Language::Lua),
                        "Rust" => Some(Language::Rust),
                        "Java" => Some(Language::Java),
                        "Kotlin" => Some(Language::Kotlin),
                        "JavaScript" => Some(Language::JavaScript),
                        "TypeScript" => Some(Language::TypeScript),
                        _ => None,
                    })
                    .collect();
                let discord_id = parts[2].to_string();
                return Ok(User {
                    username,
                    languages,
                    discord_id,
                });
            }
        }
        Err(DatabaseError::UserNotFound)
    }

    pub fn save_to_csv(&self, file_path: &str) -> Result<(), DatabaseError> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 1 && parts[0] == self.username {
                return Err(DatabaseError::UserAlreadyExists);
            }
        }

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)?;
        let languages: Vec<String> = self.languages.iter().map(|l| format!("{:?}", l)).collect();
        let line = format!(
            "{},{},{}\n",
            self.username,
            languages.join("|"),
            self.discord_id,
        );
        file.write_all(line.as_bytes())?;
        Ok(())
    }

    pub fn remove_user(file_path: &str, username: &str) -> io::Result<()> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
        let filtered_lines: Vec<String> = lines
            .into_iter()
            .filter(|line| {
                let parts: Vec<&str> = line.split(',').collect();
                parts.len() < 3 || parts[0] != username
            })
            .collect();
        let mut file = File::create(file_path)?;
        for line in filtered_lines {
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }

    pub fn to_string(&self) -> String {
        let languages_str: Vec<String> =
            self.languages.iter().map(|l| format!("{:?}", l)).collect();
        format!(
            "User {{ username: {}, languages: [{}], discord_id: {} }}",
            self.username,
            languages_str.join(", "),
            self.discord_id
        )
    }
}
