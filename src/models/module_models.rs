use diesel::prelude::*;
use diesel::{Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use super::models::{Model};

use crate::schema::modules;

#[derive(Debug, Serialize, Deserialize, Queryable, PartialEq, Clone)]
pub struct Module {
    pub module_id: i32,
    pub module_type_id: i32,
    pub page_url: String,
    pub content: String,
}

#[derive(Insertable, AsChangeset, Deserialize, Serialize)]
#[table_name = "modules"]
pub struct MutModule {
    pub module_id: Option<i32>,
    pub module_type_id: i32,
    pub page_url: String,
    pub content: Option<String>,
}

impl Model<Module, MutModule, i32> for Module {
    fn create(
        new_module: &MutModule,
        db: &MysqlConnection,
    ) -> Result<usize, diesel::result::Error> {
        Ok(diesel::insert_or_ignore_into(modules::table)
            .values(new_module)
            .execute(db)?)
    }

    fn read_one(mod_id: i32, db: &MysqlConnection) -> Result<Self, diesel::result::Error> {
        use modules::dsl::module_id;

        Ok(modules::table
            .filter(module_id.eq(mod_id))
            .first::<Self>(db)?)
    }

    fn read_all(db: &MysqlConnection) -> Result<Vec<Self>, diesel::result::Error> {
        Ok(modules::table.load::<Module>(db)?)
    }

    fn delete(mod_id: i32, db: &MysqlConnection) -> Result<usize, diesel::result::Error> {
        use crate::schema::modules::dsl::module_id;
        use crate::schema::modules::dsl::modules;

        Ok(diesel::delete(modules.filter(module_id.eq(mod_id))).execute(db)?)
    }

    fn update(
        mod_id: i32,
        new_module: &MutModule,
        db: &MysqlConnection,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::modules::dsl::module_id;
        use crate::schema::modules::dsl::modules;

        Ok(diesel::update(modules.filter(module_id.eq(mod_id)))
            .set(new_module)
            .execute(db)?)
    }
}
