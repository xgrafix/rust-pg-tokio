INSERT INTO book."book"(title, isbn, author, category)
VALUES ($1, $2, $3, $4)
RETURNING $table_fields;