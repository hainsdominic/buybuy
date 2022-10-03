use crate::db::establish_connection;
use async_graphql::*;

use super::{
    products_model::{CreateProductInput, DeleteProductInput, FindProductInput, Product},
    products_service::{create_product, delete_product, find_all_products, find_product},
};

#[derive(Default)]
pub struct ProductsQuery;

#[Object]
impl ProductsQuery {
    async fn products(&self) -> Result<Vec<Product>> {
        let connection = &mut establish_connection();
        Ok(find_all_products(connection))
    }

    async fn product(&self, input: FindProductInput) -> Result<Product> {
        let connection = &mut establish_connection();
        Ok(find_product(connection, input))
    }
}

#[derive(Default)]
pub struct ProductsMutation;

#[Object]
impl ProductsMutation {
    async fn create_product(&self, input: CreateProductInput) -> Result<Product> {
        let connection = &mut establish_connection();
        Ok(create_product(connection, input))
    }

    async fn delete_product(&self, input: DeleteProductInput) -> Result<usize> {
        let connection = &mut establish_connection();
        Ok(delete_product(connection, input)?)
    }
}
