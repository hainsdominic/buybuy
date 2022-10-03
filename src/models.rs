use crate::{
    products::products_resolver::{ProductsMutation, ProductsQuery},
    shops::shops_resolver::{ShopsMutation, ShopsQuery},
};
use async_graphql::*;
pub type BuybuySchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(MergedObject, Default)]
pub struct QueryRoot(ShopsQuery, ProductsQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(ShopsMutation, ProductsMutation);
