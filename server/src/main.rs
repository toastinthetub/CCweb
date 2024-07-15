mod server;
mod user;

use axum::handler;

use axum::{
    response::Html,
    routing::{get, post},
    Router,
};
use server::delete_post_handler;

#[tokio::main]
async fn main() {
    let languages: Vec<crate::user::Language> =
        vec![crate::user::Language::C, crate::user::Language::Rust];

    // let user = crate::user::User::create_user(
    //     Some("fork".to_owned()),
    //     Some(languages),
    //     Some("ForkInToaster".to_owned()),
    // )
    // .unwrap();

    // println!("{:?}", user);
    // user.save_to_csv(crate::server::FILEPATH).unwrap();

    let app = Router::new()
        .route(
            "/:key/:mode/:user/:languages/:discordid",
            get(crate::server::create_post_handler).post(crate::server::create_post_handler),
        )
        .route("/:key/:user/", get(crate::server::get_handler))
        .route(
            "/:key/:mode/:user",
            get(delete_post_handler).post(delete_post_handler),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
