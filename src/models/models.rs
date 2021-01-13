use actix_web::web;
use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
    MysqlConnection,
};
use std::{fs::File, io::BufReader};

use crate::{config_controllers::LocalConfig, errors_middleware::CustomHttpError};

/// CRUD implementation.
pub trait Model<TQueryable, TMutable, TPrimary> {
    fn create(new: &TMutable, db: &MysqlConnection) -> Result<usize, diesel::result::Error>;
    fn read_one(id: TPrimary, db: &MysqlConnection) -> Result<TQueryable, diesel::result::Error>;
    fn read_all(db: &MysqlConnection) -> Result<Vec<TQueryable>, diesel::result::Error>;
    fn update(
        id: TPrimary,
        new: &TMutable,
        db: &MysqlConnection,
    ) -> Result<usize, diesel::result::Error>;
    fn delete(id: TPrimary, db: &MysqlConnection) -> Result<usize, diesel::result::Error>;
}

/// Trait that enforces a  Model to be joinable if that is desired.
/// Usually used for inner joins in this program.
/// If implemented another way, make sure to follow the generic labelling.
/// First parameter MUST be the left table, and second parameter MUST be the right table.
pub trait Joinable<TLeft, TRight, TPrimary> {
    fn read_one_join_on(
        id: TPrimary,
        db: &MysqlConnection,
    ) -> Result<Vec<(TLeft, TRight)>, diesel::result::Error>;
}

pub type MySQLPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MySQLPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn establish_database_connection() -> Option<MySQLPool> {
    let config_file = File::open("./rcms.json").expect("Failed to open config file.");
    let reader = BufReader::new(config_file);
    let conf: LocalConfig = serde_json::from_reader(reader).expect("Failed to read config file.");
    let db_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        conf.mysql_username?,
        conf.mysql_password?,
        conf.mysql_url?,
        conf.mysql_port?,
        conf.mysql_database?
    );

    Some(init_pool(&db_url).expect("Failed to create pool."))
}

// https://dev.to/werner/practical-rust-web-development-connection-pool-46f4
pub fn init_pool(db_url: &str) -> Result<MySQLPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(db_url);
    Pool::builder().max_size(10).build(manager)
}

pub fn pool_handler(pool: web::Data<MySQLPool>) -> Result<MySQLPooledConnection, CustomHttpError> {
    pool.get().or(Err(CustomHttpError::Unknown))
}
