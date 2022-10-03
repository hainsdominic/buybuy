use crate::db::establish_connection;
use crate::shops::shops_model::ShopWithProducts;
use async_graphql::*;

use super::{
    shops_model::{CreateShopInput, DeleteShopInput, FindShopInput},
    shops_service::{create_shop, delete_shop, find_all_shops, find_shop},
};

#[derive(Default)]
pub struct ShopsQuery;

#[Object]
impl ShopsQuery {
    async fn shops(&self) -> Result<Vec<ShopWithProducts>> {
        let connection = &mut establish_connection();
        Ok(find_all_shops(connection))
    }

    async fn shop(&self, input: FindShopInput) -> Result<ShopWithProducts> {
        let connection = &mut establish_connection();
        Ok(find_shop(connection, input))
    }
}

#[derive(Default)]
pub struct ShopsMutation;

#[Object]
impl ShopsMutation {
    async fn create_shop(&self, input: CreateShopInput) -> Result<ShopWithProducts> {
        let connection = &mut establish_connection();
        Ok(create_shop(connection, input))
    }

    async fn delete_shop(&self, input: DeleteShopInput) -> Result<usize> {
        let connection = &mut establish_connection();
        Ok(delete_shop(connection, input)?)
    }
}
