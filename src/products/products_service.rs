use diesel::prelude::*;
use diesel::PgConnection;

use crate::products::products_model::NewProduct;

use super::products_model::{CreateProductInput, DeleteProductInput, FindProductInput, Product};

pub fn find_all_products(connection: &mut PgConnection) -> Vec<Product> {
    use crate::schema::products::dsl::*;
    products
        .load::<Product>(connection)
        .expect("Error loading products")
}

pub fn find_product(connection: &mut PgConnection, input: FindProductInput) -> Product {
    use crate::schema::products::dsl::*;
    products
        .find(input.id)
        .first::<Product>(connection)
        .expect("Error loading product")
}

pub fn create_product(conn: &mut PgConnection, input: CreateProductInput) -> Product {
    use crate::schema::products;

    let new_product = NewProduct {
        name: &input.name,
        price: input.price,
        shop_id: input.shop_id,
    };

    diesel::insert_into(products::table)
        .values(&new_product)
        .get_result(conn)
        .expect("Error saving new Product")
}

pub fn delete_product(
    conn: &mut PgConnection,
    input: DeleteProductInput,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::products::dsl::*;

    diesel::delete(products.filter(id.eq(input.id))).execute(conn)
}
