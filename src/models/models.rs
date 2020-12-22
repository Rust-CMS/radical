use diesel::{Connection, MysqlConnection};
use dotenv::dotenv;
use std::env;

/// CRUD implementation.
pub trait Model<T, G> {
    fn create(new: &G) -> Result<usize, diesel::result::Error>;
    fn read_one(id: i32) -> Result<T, diesel::result::Error>;
    fn read_all() -> Result<Vec<T>, diesel::result::Error>;
    fn update(id: i32, new: &G) -> Result<usize, diesel::result::Error>;
    fn delete(id: i32) -> Result<usize, diesel::result::Error>;
}

pub fn establish_database_connection() -> MysqlConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("Environment variable DATABASE_URL must be set.");
    MysqlConnection::establish(&db_url).expect(&format!(
        "Error connecting to database using URL {}",
        &db_url
    ))
}
