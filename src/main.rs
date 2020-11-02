
use actix_web::{App, HttpServer};
use mysql::Pool;
#[macro_use]
extern crate dotenv_codegen;

pub mod routes;
pub mod controllers;
pub mod database;
pub mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("{}", dotenv!("TOKEN"));
    println!("{}", dotenv!("DATABASE"));
    let url = dotenv!("DATABASE");

    let pool = Pool::new(url).unwrap();


    println!("{}", "UP");
    
    HttpServer::new(move || {
        App::new()
        .data(pool.clone())
        .service(routes::user::post)
        .service(routes::user::get)
    }
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}