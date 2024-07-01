use bcrypt::{hash, verify, BcryptError};
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    ident: String,
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
    pub fn new(
        username: String,
        password: String,
        file_path: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut ident;
        loop {
            ident = Uuid::new_v4().to_string();
            if !Self::ident_exists(file_path, &ident)? {
                break;
            }
        }

        if Self::username_exists(file_path, &username)? {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Username already exists",
            )));
        }

        let password_hash = hash(password, bcrypt::DEFAULT_COST)
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?;

        Ok(User {
            ident,
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
            "{},{},{},{}\n",
            self.ident,
            self.username,
            self.password_hash,
            languages.join("|")
        );
        file.write_all(line.as_bytes())
    }

    pub fn lookup_user(file_path: &str, ident: &str) -> io::Result<Option<User>> {
        let file = OpenOptions::new().read(true).open(file_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 4 && parts[0] == ident {
                let ident = parts[0].to_string();
                let username = parts[1].to_string();
                let password_hash = parts[2].to_string();
                let languages = parts[3]
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
                    ident,
                    username,
                    password_hash,
                    languages,
                }));
            }
        }
        Ok(None)
    }

    fn ident_exists(file_path: &str, ident: &str) -> io::Result<bool> {
        let file = OpenOptions::new().read(true).open(file_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 4 && parts[0] == ident {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn username_exists(file_path: &str, username: &str) -> io::Result<bool> {
        let file = OpenOptions::new().read(true).open(file_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 4 && parts[1] == username {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
