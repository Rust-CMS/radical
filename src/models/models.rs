use actix_web::web;
use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
    Connection, MysqlConnection,
};
use dotenv::dotenv;
use std::env;

use crate::errors_middleware::CustomHttpError;

/// CRUD implementation.
pub trait Model<TQueryable, TMutable> {
    fn create(new: &TMutable, db: &MysqlConnection) -> Result<usize, diesel::result::Error>;
    fn read_one(id: i32, db: &MysqlConnection) -> Result<TQueryable, diesel::result::Error>;
    fn read_all(db: &MysqlConnection) -> Result<Vec<TQueryable>, diesel::result::Error>;
    fn update(
        id: i32,
        new: &TMutable,
        db: &MysqlConnection,
    ) -> Result<usize, diesel::result::Error>;
    fn delete(id: i32, db: &MysqlConnection) -> Result<usize, diesel::result::Error>;
}

/// Trait that enforces a  Model to be joinable if that is desired.
/// Usually used for inner joins in this program.
/// If implemented another way, make sure to follow the generic labelling.
/// First parameter MUST be the left table, and second parameter MUST be the right table.
pub trait Joinable<TLeft, TRight> {
    fn read_one_join_on(
        id: i32,
        db: &MysqlConnection,
    ) -> Result<Vec<(TLeft, TRight)>, diesel::result::Error>;
}

pub type MySQLPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MySQLPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn establish_database_connection() -> MySQLPool {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("Environment variable DATABASE_URL must be set.");
    init_pool(&db_url).expect("Failed to create pool.")
}

// https://dev.to/werner/practical-rust-web-development-connection-pool-46f4
pub fn init_pool(db_url: &str) -> Result<MySQLPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(db_url);
    Pool::builder().max_size(10).build(manager)
}

pub fn pool_handler(pool: web::Data<MySQLPool>) -> Result<MySQLPooledConnection, CustomHttpError> {
    pool.get().or(Err(CustomHttpError::Unknown))
}
