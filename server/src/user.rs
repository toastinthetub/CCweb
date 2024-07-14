use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub languages: Vec<Language>,
    pub discord_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Language {
    C,
    CPP,
    CSharp,
    Java,
    JavaScript,
    TypeScript,
    Python,
    Ruby,
    Rust,
    Go,
    Swift,
    Kotlin,
    Lua,
    PHP,
    Perl,
    ObjectiveC,
    Scala,
    Haskell,
    Shell,
    R,
    Julia,
    Dart,
    VB,
    FSharp,
    Lisp,
    Prolog,
    Assembly,
    SQL,
    HTML,
    CSS,
    Verilog,
    Matlab,
    Cobol,
    Fortran,
    Ada,
    Delphi,
    Smalltalk,
    Erlang,
    Tcl,
    Scheme,
    Apex,
    ApexTrigger,
    CoffeeScript,
    Elm,
    PureScript,
    Crystal,
    Elixir,
    Raku,
    Hack,
    VHDL,
    BadLanguage,
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
        let lowercased_input = input.trim().to_lowercase();
        match lowercased_input.as_str() {
            "c" => Ok(Language::C),
            "cpp" => Ok(Language::CPP),
            "csharp" | "c#" => Ok(Language::CSharp),
            "java" => Ok(Language::Java),
            "javascript" | "js" => Ok(Language::JavaScript),
            "typescript" | "ts" => Ok(Language::TypeScript),
            "python" | "py" => Ok(Language::Python),
            "ruby" => Ok(Language::Ruby),
            "rust" => Ok(Language::Rust),
            "go" | "golang" => Ok(Language::Go),
            "swift" => Ok(Language::Swift),
            "kotlin" => Ok(Language::Kotlin),
            "lua" => Ok(Language::Lua),
            "php" => Ok(Language::PHP),
            "perl" => Ok(Language::Perl),
            "objc" | "objective-c" => Ok(Language::ObjectiveC),
            "scala" => Ok(Language::Scala),
            "haskell" => Ok(Language::Haskell),
            "shell" | "bash" => Ok(Language::Shell),
            "r" => Ok(Language::R),
            "julia" => Ok(Language::Julia),
            "dart" => Ok(Language::Dart),
            "vb" | "visualbasic" => Ok(Language::VB),
            "fsharp" => Ok(Language::FSharp),
            "lisp" => Ok(Language::Lisp),
            "prolog" => Ok(Language::Prolog),
            "assembly" => Ok(Language::Assembly),
            "sql" => Ok(Language::SQL),
            "html" => Ok(Language::HTML),
            "css" => Ok(Language::CSS),
            "verilog" => Ok(Language::Verilog),
            "vhdl" => Ok(Language::VHDL),
            "matlab" => Ok(Language::Matlab),
            "cobol" => Ok(Language::Cobol),
            "fortran" => Ok(Language::Fortran),
            "ada" => Ok(Language::Ada),
            "delphi" => Ok(Language::Delphi),
            "smalltalk" => Ok(Language::Smalltalk),
            "erlang" => Ok(Language::Erlang),
            "tcl" => Ok(Language::Tcl),
            "scheme" => Ok(Language::Scheme),
            "apex" => Ok(Language::Apex),
            "apextrigger" => Ok(Language::ApexTrigger),
            "coffeescript" => Ok(Language::CoffeeScript),
            "elm" => Ok(Language::Elm),
            "purescript" => Ok(Language::PureScript),
            "crystal" => Ok(Language::Crystal),
            "elixir" => Ok(Language::Elixir),
            "raku" => Ok(Language::Raku),
            "hack" => Ok(Language::Hack),
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

    pub fn add_language(
        &mut self,
        new_languages: Vec<Language>,
        file_path: &str,
    ) -> Result<(), DatabaseError> {
        for language in new_languages {
            if !self.languages.contains(&language) {
                self.languages.push(language);
            }
        }

        self.update_user(file_path)?;

        Ok(())
    }

    pub fn remove_language(
        &mut self,
        languages_to_remove: Vec<Language>,
        file_path: &str,
    ) -> Result<(), DatabaseError> {
        self.languages.retain(|l| !languages_to_remove.contains(l));

        self.update_user(file_path)?;

        Ok(())
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
                    .filter_map(|s| Language::from_str(s.trim()).ok())
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
            "{},{},{}",
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
    fn update_user(&self, file_path: &str) -> Result<(), DatabaseError> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let mut lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
        let mut found = false;

        for line in lines.iter_mut() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 1 && parts[0] == self.username {
                *line = format!(
                    "{},{},{}\n",
                    self.username,
                    self.languages_str(),
                    self.discord_id
                );
                found = true;
                break;
            }
        }

        if !found {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(file_path)?;
            let line = format!(
                "{},{},{}\n",
                self.username,
                self.languages_str(),
                self.discord_id,
            );
            file.write_all(line.as_bytes())?;
        } else {
            let mut file = File::create(file_path)?;
            for line in lines {
                writeln!(file, "{}", line)?;
            }
        }

        Ok(())
    }
    fn languages_str(&self) -> String {
        self.languages
            .iter()
            .map(|l| format!("{:?}", l))
            .collect::<Vec<_>>()
            .join("|")
    }
}
