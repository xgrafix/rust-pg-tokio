pub mod db {
    use crate::{MyError, Book, BookNoId};
    use deadpool_postgres::Client;
    use tokio_pg_mapper::FromTokioPostgresRow;

     pub async fn add_book(client: &Client, book_info: BookNoId) -> Result<BookNoId, MyError> {
        let _stmt = include_str!("../sql/add_book.sql");
        let _stmt = _stmt.replace("$table_fields", &BookNoId::sql_table_fields());
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
            .map(|row| BookNoId::from_row_ref(row).unwrap())
            .collect::<Vec<BookNoId>>()
            .pop()
            .ok_or(MyError::NotFound) // more applicable for SELECTs
    }

    pub async fn get_book(client: &Client) -> Result<Vec<Book>, MyError> {
        println!("From get_book function");

        let _stmt = client.prepare("SELECT * FROM book.book").await.unwrap();

        let books = client.query(&_stmt, &[])
            .await
            .expect("Error")
            .iter()
            .map(|row| Book::from_row_ref(row).unwrap())
            .collect::<Vec<Book>>();

        Ok(books)
    }

    pub async fn get_book_id(client: &Client, book_id: i32) -> Result<Book, MyError> {
        println!("From get_book_id function");

        let _stmt = client.prepare("SELECT * FROM book.book WHERE book_id = $1").await.unwrap();

        client.query(&_stmt, &[&book_id])
            .await?
            .iter()
            .map(|row| Book::from_row_ref(row).unwrap())
            .collect::<Vec<Book>>()
            .pop()
            .ok_or(MyError::NotFound)
    }

}
