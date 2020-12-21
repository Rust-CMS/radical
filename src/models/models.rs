use async_trait::async_trait;
use diesel::{Connection, MysqlConnection};
use dotenv::dotenv;
use std::env;

use super::{MutPage, Page};

/// CRUD implementation.
#[async_trait]
pub trait Model {
    async fn create(new_page: &MutPage);
    async fn read_one(page_id: i32) -> Page;
    async fn read_all() -> Vec<Page>;
    async fn update(id: i32, new_page: &MutPage);
    async fn delete(id: i32);
}

pub fn establish_database_connection() -> MysqlConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("Environment variable DATABASE_URL must be set.");
    MysqlConnection::establish(&db_url).expect(&format!(
        "Error connecting to database using URL {}",
        &db_url
    ))
}