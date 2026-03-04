// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Text,
    }
}

diesel::table! {
    pages (book_id, number) {
        book_id -> Int4,
        number -> Int4,
        content -> Nullable<Text>,
    }
}

diesel::joinable!(pages -> books (book_id));

diesel::allow_tables_to_appear_in_same_query!(books, pages,);
