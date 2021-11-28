pub mod config_models;
pub mod module_models;
pub mod page_models;
pub mod user_models;

use actix_web::web;
use diesel::{MysqlConnection, query_builder::AsChangeset, r2d2::{ConnectionManager, Pool, PoolError, PooledConnection}};

use crate::services::errors_service::CustomHttpError;

use self::config_models::LocalConfig;

pub type MySQLPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MySQLPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

/// CRUD implementation.
/// TQueryable: The queryable struct.
/// TMutable: The struct that represents the mutable columns in the table.
/// TPrimary: The primary key type.
/// TDto: The DTO object that will be sent back to the user.
pub trait Model<TQueryable, TMutable: AsChangeset, TPrimary, TDto = TQueryable> {
    fn create(new: &TMutable, db: &MysqlConnection) -> Result<usize, diesel::result::Error>;
    fn read_one(id: TPrimary, db: &MysqlConnection) -> Result<TDto, diesel::result::Error>;
    fn read_all(db: &MysqlConnection) -> Result<Vec<TDto>, diesel::result::Error>;
    fn update(
        id: TPrimary,
        new: &TMutable,
        db: &MysqlConnection,
    ) -> Result<usize, diesel::result::Error>;
    fn delete(id: TPrimary, db: &MysqlConnection) -> Result<usize, diesel::result::Error>;
}

pub trait DTO<TColumns> {
    fn columns() -> TColumns;
}

/// Trait that enforces a  Model to be joinable if that is desired.
/// This should use associations rather than Left or Right join.
/// https://docs.diesel.rs/diesel/associations/index.html
pub trait Joinable<TLeft, TRight, TPrimary> {
    fn read_one_join_on(
        id: TPrimary,
        db: &MysqlConnection,
    ) -> Result<(TLeft, Vec<TRight>), diesel::result::Error>;
}

pub fn format_connection_string(conf: LocalConfig) -> String {
    match conf.mysql_url {
        Some(mysql_url) => {
            format!(
                "mysql://{}:{}@{}:{}/{}",
                conf.mysql_username,
                conf.mysql_password,
                mysql_url,
                conf.mysql_port.unwrap(),
                conf.mysql_database
            )
        }
        None if std::env::var("MYSQL_UNIX_PORT").is_ok() => {
            format!(
                "mysql://{}:{}@/{}",
                conf.mysql_username,
                conf.mysql_password,
                conf.mysql_database
            )
        }
        None => {
            panic!("Must supply one of the following: [mysql_url], [sql_name | socket_dir]")
        }
    }
}

pub fn establish_database_connection(conf: LocalConfig) -> Option<MySQLPool> {
    let db_url = format_connection_string(conf);

    Some(init_pool(&db_url).expect("Failed to create pool."))
}

pub fn init_connection(db_url: &str) -> ConnectionManager<diesel::MysqlConnection> {
    ConnectionManager::<MysqlConnection>::new(db_url)
}

// https://dev.to/werner/practical-rust-web-development-connection-pool-46f4
pub fn init_pool(db_url: &str) -> Result<MySQLPool, PoolError> {
    let manager = init_connection(db_url);
    Pool::builder().max_size(2).build(manager)
}

pub fn pool_handler(pool: web::Data<MySQLPool>) -> Result<MySQLPooledConnection, CustomHttpError> {
    pool.get().or(Err(CustomHttpError::BadRequest))
}
