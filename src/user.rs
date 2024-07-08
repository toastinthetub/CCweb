use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    username: String,
    password_hash: String,
    languages: Vec<Language>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    C,
    CPP,
    Rust,
    Java,
    Kotlin,
    JavaScript,
    TypeScript,
}

impl User {
    pub fn new(username: String, password: String) -> Result<Self, Box<dyn std::error::Error>> {
        let password_hash = hash(password, DEFAULT_COST)?;

        Ok(User {
            username,
            password_hash,
            languages: Vec::new(),
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

    pub fn authenticate(&self, password: &str) -> bool {
        match verify(password, &self.password_hash) {
            Ok(valid) => valid,
            Err(_) => false,
        }
    }

    pub fn save_to_csv(&self, file_path: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)?;
        let languages: Vec<String> = self.languages.iter().map(|l| format!("{:?}", l)).collect();
        let line = format!(
            "{},{},{}\n",
            self.username,
            self.password_hash,
            languages.join("|")
        );
        file.write_all(line.as_bytes())
    }

    pub fn lookup_user(file_path: &str, username: &str) -> io::Result<Option<User>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 3 && parts[0] == username {
                let username = parts[0].to_string();
                let password_hash = parts[1].to_string();
                let languages = parts[2]
                    .split('|')
                    .filter_map(|s| match s {
                        "C" => Some(Language::C),
                        "CPP" => Some(Language::CPP),
                        "Rust" => Some(Language::Rust),
                        "Java" => Some(Language::Java),
                        "Kotlin" => Some(Language::Kotlin),
                        "JavaScript" => Some(Language::JavaScript),
                        "TypeScript" => Some(Language::TypeScript),
                        _ => None,
                    })
                    .collect();
                return Ok(Some(User {
                    username,
                    password_hash,
                    languages,
                }));
            }
        }
        Ok(None)
    }

    pub fn to_string(&self) -> String {
        let languages_str: Vec<String> =
            self.languages.iter().map(|l| format!("{:?}", l)).collect();
        format!(
            "User {{ username: {}, password_hash: {}, languages: [{}] }}",
            self.username,
            self.password_hash,
            languages_str.join(", ")
        )
    }
}
