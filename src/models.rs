pub mod models {
    use serde::{Deserialize, Serialize};
    use tokio_pg_mapper_derive::PostgresMapper;

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "book")] // singular 'user' is a keyword..
    pub struct Book {
        pub book_id: i32,
        pub title: String,
        pub isbn: String,
        pub author: String,
        pub category: String,
    }

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "book")] // singular 'user' is a keyword..
    pub struct BookNoId {
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