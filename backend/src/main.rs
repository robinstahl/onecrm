use actix_web::{web, App, HttpServer, Responder};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new())
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
