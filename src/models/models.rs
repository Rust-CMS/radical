use std::env;
use dotenv::dotenv;
use diesel::{Connection, MysqlConnection};

/// CRUD implementation.
pub trait Model<TQueryable, TMutable> {
    fn create(new: &TMutable) -> Result<usize, diesel::result::Error>;
    fn read_one(id: i32) -> Result<TQueryable, diesel::result::Error>;
    fn read_all() -> Result<Vec<TQueryable>, diesel::result::Error>;
    fn update(id: i32, new: &TMutable) -> Result<usize, diesel::result::Error>;
    fn delete(id: i32) -> Result<usize, diesel::result::Error>;
}

/// Trait that enforces a  Model to be joinable if that is desired.
/// Usually used for inner joins in this program.
/// If implemented another way, make sure to follow the generic labelling.
/// First parameter MUST be the left table, and second parameter MUST be the right table.
pub trait Joinable<TLeft, TRight> {
    fn read_one_join_on(id: i32) -> Result<Vec<(TLeft, TRight)>, diesel::result::Error>;
}

pub fn establish_database_connection() -> MysqlConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("Environment variable DATABASE_URL must be set.");
    MysqlConnection::establish(&db_url).expect(&format!(
        "Error connecting to database using URL {}",
        &db_url
    ))
}
