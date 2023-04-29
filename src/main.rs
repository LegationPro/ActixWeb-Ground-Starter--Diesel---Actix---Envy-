use std::io::{stdin, Read};

use actix_web::{get, App, HttpServer, Responder};

mod api;
mod app_env;
mod db;
mod models;
mod schema;

#[get("/")]
async fn index() -> impl Responder {
    "Hello world"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use crate::api::show_posts;
    

    let config = app_env::init();

    let mut conn = db::establish_connection(config.database_url);

    let mut title = String::new();
    let mut body = String::new();

    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end();

    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        title, EOF
    );


    // stdin().read_to_string(&mut body).unwrap();

    stdin().read_to_string(&mut body).unwrap();

    show_posts::create_post(&mut conn, title, body.as_str());

    show_posts::show_posts(&mut conn);
    


    HttpServer::new(|| App::new().service(index))
        .bind((config.host, config.port))?
        .run()
        .await
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";