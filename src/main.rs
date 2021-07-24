use std::{env};

mod config;
mod errors;
mod models;
mod db;
mod handlers;

pub use crate::config::config::Config;
pub use crate::errors::errors::MyError;
pub use crate::models::models::{Book, BookNoId};
pub use crate::db::db::{add_book, get_book, get_book_id};
pub use crate::handlers::handlers::*;

use actix_web::{web, App, HttpServer, middleware, Responder};
use dotenv::dotenv;
use tokio_postgres::NoTls;

async fn status() -> impl Responder {
    use crate::models::models::Status;
    // "{\"status:\" \"UP\"}"
    web::HttpResponse::Ok().json(Status {status: "ok".to_string()})
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();


    let config = crate::config::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(status))
            .data(pool.clone())
            // .service(web::resource("/books").route(web::post().to(add_book)))
            .service(web::resource("/books{_:/?}")
                .route(web::post().to(handlers::handlers::add_book))
                .route(web::get().to(handlers::handlers::get_book))
            )
            .service(web::resource("/books/{book_id}")
                .route(web::get().to(handlers::handlers::get_book_id))
            )
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())

    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}