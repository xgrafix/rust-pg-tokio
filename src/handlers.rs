pub mod handlers {
    use crate::{db, MyError, Book, BookNoId};
    use actix_web::{web, Error, HttpResponse};
    use deadpool_postgres::{Client, Pool};

    pub async fn add_book(
        book: web::Json<BookNoId>,
        db_pool: web::Data<Pool>,
    ) -> Result<HttpResponse, Error> {
        let book_info: BookNoId = book.into_inner();

        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

        let new_book = db::db::add_book(&client, book_info).await?;

        Ok(HttpResponse::Ok().json(new_book))
    }

    pub async fn get_book(
        // book: web::Json<Book>,
        db_pool: web::Data<Pool>,
    ) -> Result<HttpResponse, Error> {

        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

        let all_books = db::db::get_book(&client).await?;

        Ok(HttpResponse::Ok().json(all_books))
    }

    pub async fn get_book_id(
        // book: web::Json<Book>,
        path: web::Path<i32>,
        db_pool: web::Data<Pool>,
    ) -> Result<HttpResponse, Error> {
        // use crate::models::Status;


        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

        let all_books = db::db::get_book_id(&client, path.0).await?;

        println!("{}", path.0);
        Ok(HttpResponse::Ok().json(all_books))
        // Ok(web::HttpResponse::Ok().json(Status {status: "ok".to_string()}))

    }
}
