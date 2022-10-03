use crate::{products::products_model::Product, schema::shops};
use async_graphql::*;
use diesel::prelude::*;

#[derive(Queryable, Identifiable)]
#[diesel(table_name = shops)]
pub struct Shop {
    pub id: i32,
    pub name: String,
}

#[derive(SimpleObject)]
pub struct ShopWithProducts {
    pub id: i32,
    pub name: String,
    pub products: Vec<Product>,
}

#[derive(Insertable)]
#[diesel(table_name = shops)]
pub struct NewShop<'a> {
    pub name: &'a str,
}

#[derive(InputObject)]
pub struct FindShopInput {
    pub id: i32,
}

#[derive(InputObject)]
pub struct CreateShopInput {
    pub name: String,
}

#[derive(InputObject)]
pub struct DeleteShopInput {
    pub id: i32,
}
