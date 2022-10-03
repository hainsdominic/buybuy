use crate::schema::products;
use crate::shops::shops_model::Shop;
use async_graphql::*;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Associations, SimpleObject)]
#[diesel(table_name = products)]
#[diesel(belongs_to(Shop))]
pub struct Product {
    pub id: i32,
    pub shop_id: i32,
    pub name: String,
    pub price: i32,
}

#[derive(Insertable)]
#[diesel(table_name = products)]
pub struct NewProduct<'a> {
    pub name: &'a str,
    pub price: i32,
    pub shop_id: i32,
}

#[derive(InputObject)]
pub struct FindProductInput {
    pub id: i32,
}

#[derive(InputObject)]
pub struct CreateProductInput {
    pub name: String,
    pub price: i32,
    pub shop_id: i32,
}

#[derive(InputObject)]
pub struct DeleteProductInput {
    pub id: i32,
}
