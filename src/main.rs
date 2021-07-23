use std::{env};


mod config {
    pub use ::config::ConfigError;
    use serde::Deserialize;
    #[derive(Deserialize)]
    pub struct Config {
        pub server_addr: String,
        pub pg: deadpool_postgres::Config,
    }
    impl Config {
        pub fn from_env() -> Result<Self, ConfigError> {
            let mut cfg = ::config::Config::new();
            cfg.merge(::config::Environment::new())?;
            cfg.try_into()
        }
    }
}

mod models {
    use serde::{Deserialize, Serialize};
    use tokio_pg_mapper_derive::PostgresMapper;

    // #[derive(Deserialize, PostgresMapper, Serialize)]
    // #[pg_mapper(table = "users")] // singular 'user' is a keyword..
    // pub struct User {
    //     pub email: String,
    //     pub first_name: String,
    //     pub last_name: String,
    //     pub username: String,
    // }

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "book")] // singular 'user' is a keyword..
    pub struct Book {
        pub title: String,
        pub isbn: String,
        pub author: String,
        pub category: String,
    }

    #[derive(Serialize)]
    pub struct Status {
        pub status: String
    }
}


mod errors {
    use actix_web::{HttpResponse, ResponseError};
    use deadpool_postgres::PoolError;
    use derive_more::{Display, From};
    use tokio_pg_mapper::Error as PGMError;
    use tokio_postgres::error::Error as PGError;

    #[derive(Display, From, Debug)]
    pub enum MyError {
        NotFound,
        PGError(PGError),
        PGMError(PGMError),
        PoolError(PoolError),
    }
    impl std::error::Error for MyError {}

    impl ResponseError for MyError {
        fn error_response(&self) -> HttpResponse {
            match *self {
                MyError::NotFound => HttpResponse::NotFound().finish(),
                MyError::PoolError(ref err) => {
                    HttpResponse::InternalServerError().body(err.to_string())
                }
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}


mod db {
    use crate::{errors::MyError, models::Book};
    use deadpool_postgres::Client;
    use tokio_pg_mapper::FromTokioPostgresRow;

    // pub async fn add_user(client: &Client, user_info: User) -> Result<User, MyError> {
    //     let _stmt = include_str!("../sql/add_user.sql");
    //     let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    //     let stmt = client.prepare(&_stmt).await.unwrap();

    //     client
    //         .query(
    //             &stmt,
    //             &[
    //                 &user_info.email,
    //                 &user_info.first_name,
    //                 &user_info.last_name,
    //                 &user_info.username,
    //             ],
    //         )
    //         .await?
    //         .iter()
    //         .map(|row| User::from_row_ref(row).unwrap())
    //         .collect::<Vec<User>>()
    //         .pop()
    //         .ok_or(MyError::NotFound) // more applicable for SELECTs
    // }

     pub async fn add_book(client: &Client, book_info: Book) -> Result<Book, MyError> {
        let _stmt = include_str!("../sql/add_book.sql");
        let _stmt = _stmt.replace("$table_fields", &Book::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();

        client
            .query(
                &stmt,
                &[
                    &book_info.title,
                    &book_info.isbn,
                    &book_info.author,
                    &book_info.category,
                ],
            )
            .await?
            .iter()
            .map(|row| Book::from_row_ref(row).unwrap())
            .collect::<Vec<Book>>()
            .pop()
            .ok_or(MyError::NotFound) // more applicable for SELECTs
    }

    pub async fn get_book(client: &Client) -> Result<Vec<Book>, MyError> {
        let _stmt = client.prepare("SELECT * FROM book.book").await.unwrap();

        let books = client.query(&_stmt, &[])
            .await
            .expect("Error")
            .iter()
            .map(|row| Book::from_row_ref(row).unwrap())
            .collect::<Vec<Book>>();

        Ok(books)
    }
}

mod handlers {
    use crate::{db, errors::MyError, models::Book};
    use actix_web::{web, Error, HttpResponse};
    use deadpool_postgres::{Client, Pool};

    pub async fn add_book(
        book: web::Json<Book>,
        db_pool: web::Data<Pool>,
    ) -> Result<HttpResponse, Error> {
        let book_info: Book = book.into_inner();

        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

        let new_book = db::add_book(&client, book_info).await?;

        Ok(HttpResponse::Ok().json(new_book))
    }

    pub async fn get_book(
        // book: web::Json<Book>,
        db_pool: web::Data<Pool>,
    ) -> Result<HttpResponse, Error> {

        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

        let all_books = db::get_book(&client).await?;

        Ok(HttpResponse::Ok().json(all_books))
    }
}

// mod models;

use actix_web::{web, App, HttpServer, middleware, Responder};
use dotenv::dotenv;
use handlers::add_book;
use handlers::get_book;
use tokio_postgres::NoTls;


async fn status() -> impl Responder {
    use crate::models::Status;
    // "{\"status:\" \"UP\"}"
    web::HttpResponse::Ok().json(Status {status: "ok".to_string()})
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();


    let config = crate::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
        // enable logger - always register actix-web Logger middleware last
            .route("/", web::get().to(status))
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            // .service(web::resource("/books").route(web::post().to(add_book)))
            .service(web::resource("/books")
                .route(web::post().to(add_book))
                .route(web::get().to(get_book))
            )

    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}