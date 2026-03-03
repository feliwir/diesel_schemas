-- Your SQL goes here
CREATE TABLE pages (
    book_id INTEGER NOT NULL REFERENCES books(id) ON DELETE CASCADE,
    number SERIAL NOT NULL,
    content TEXT,
    PRIMARY KEY (book_id, number)
);