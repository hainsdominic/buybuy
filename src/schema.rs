// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Int4,
        shop_id -> Int4,
        name -> Varchar,
        price -> Int4,
    }
}

diesel::table! {
    shops (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    products,
    shops,
);
