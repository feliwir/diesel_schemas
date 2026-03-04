-- Your SQL goes here
CREATE TABLE pages (
    book_id INTEGER NOT NULL,
    number INTEGER NOT NULL,
    content TEXT,
    PRIMARY KEY (book_id, number),
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE
);
