use diesel::prelude::*;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::models::Model;

use crate::schema::web_config;

#[derive(Queryable, Deserialize, Serialize, PartialEq, Clone)]
pub struct Config {
    config_key: String,
    config_val: String,
}

#[derive(Insertable, AsChangeset, Deserialize, Serialize)]
#[table_name = "web_config"]
pub struct MutConfig {
    config_key: String,
    config_val: String,
}

impl Model<Config, MutConfig, String> for Config {
    fn create(
        new: &MutConfig,
        db: &diesel::MysqlConnection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::insert_into(web_config::table)
            .values(new)
            .execute(db)
    }

    fn read_one(id: String, db: &diesel::MysqlConnection) -> Result<Config, diesel::result::Error> {
        use web_config::dsl::config_key;

        web_config::table
            .filter(config_key.eq(id))
            .first::<Self>(db)
    }

    fn read_all(db: &diesel::MysqlConnection) -> Result<Vec<Config>, diesel::result::Error> {
        web_config::table.load::<Self>(db)
    }

    fn update(
        id: String,
        new: &MutConfig,
        db: &diesel::MysqlConnection,
    ) -> Result<usize, diesel::result::Error> {
        use web_config::dsl::config_key;
        diesel::update(web_config::table)
            .filter(config_key.eq(id))
            .set(new)
            .execute(db)
    }

    fn delete(id: String, db: &diesel::MysqlConnection) -> Result<usize, diesel::result::Error> {
        use web_config::dsl::config_key;

        diesel::delete(web_config::table)
            .filter(config_key.eq(id))
            .execute(db)
    }
}
