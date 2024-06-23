pub struct User {
    ident: String,
    username: String,
    languages: Vec<Language>,
}

pub enum Language {
    C,
    CPP,
    Rust,
    Java,
    Kotlin,
    JavaScript,
    TypeScript,
}
