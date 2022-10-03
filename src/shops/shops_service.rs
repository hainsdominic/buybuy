use diesel::prelude::*;
use diesel::PgConnection;

use crate::products::products_model::Product;
use crate::shops::shops_model::NewShop;

use super::shops_model::CreateShopInput;
use super::shops_model::DeleteShopInput;
use super::shops_model::FindShopInput;
use super::shops_model::Shop;
use super::shops_model::ShopWithProducts;

pub fn find_all_shops(connection: &mut PgConnection) -> Vec<ShopWithProducts> {
    use crate::schema::shops::dsl::*;
    shops
        .load::<Shop>(connection)
        .expect("Error loading shops")
        .into_iter()
        .map(|shop| ShopWithProducts {
            id: shop.id,
            name: shop.name.clone(),
            products: Product::belonging_to(&shop)
                .load::<Product>(connection)
                .expect("Error loading products"),
        })
        .collect()
}

pub fn find_shop(connection: &mut PgConnection, input: FindShopInput) -> ShopWithProducts {
    use crate::schema::shops::dsl::*;

    ShopWithProducts {
        id: shops
            .find(input.id)
            .first::<Shop>(connection)
            .expect("Error loading shop")
            .id,
        name: shops
            .find(input.id)
            .first::<Shop>(connection)
            .expect("Error loading shop")
            .name,
        products: Product::belonging_to(
            &shops
                .find(input.id)
                .first::<Shop>(connection)
                .expect("Error loading shop"),
        )
        .load::<Product>(connection)
        .expect("Error loading products"),
    }
}

pub fn create_shop(conn: &mut PgConnection, input: CreateShopInput) -> ShopWithProducts {
    use crate::schema::shops;

    let new_shop = NewShop { name: &input.name };

    let shop: Shop = diesel::insert_into(shops::table)
        .values(&new_shop)
        .get_result(conn)
        .expect("Error saving new Shop");

    ShopWithProducts {
        id: shop.id,
        name: shop.name,
        products: vec![],
    }
}

pub fn delete_shop(
    conn: &mut PgConnection,
    input: DeleteShopInput,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::shops::dsl::*;

    diesel::delete(shops.filter(id.eq(input.id))).execute(conn)
}
