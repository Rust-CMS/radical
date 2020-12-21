use diesel::{Connection, MysqlConnection};
use dotenv::dotenv;
use std::env;

/// CRUD implementation.
pub trait Model<T, G> {
    fn create(new: &G);
    fn read_one(id: i32) -> T;
    fn read_all() -> Vec<T>;
    fn update(id: i32, new: &G);
    fn delete(id: i32);
}

pub fn establish_database_connection() -> MysqlConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("Environment variable DATABASE_URL must be set.");
    MysqlConnection::establish(&db_url).expect(&format!(
        "Error connecting to database using URL {}",
        &db_url
    ))
}