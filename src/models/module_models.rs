use diesel::{Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[path = "../schemas/schema.rs"]
mod schema;

use super::models::{Model, establish_database_connection};

use schema::modules;

#[derive(Debug, Serialize, Deserialize, Queryable, PartialEq, Clone)]
pub struct Module {
    module_id: i32,
    module_type_id: i32,
    page_id: i32,
    content: Option<String>
}

#[derive(Insertable, AsChangeset, Deserialize, Serialize)]
#[table_name = "modules"]
pub struct MutModule {
    pub module_id: Option<i32>,
    pub module_type_id: i32,
    pub page_id: i32,
    pub content: Option<String>
}

impl Model<Module, MutModule> for Module {
    fn create(new_module: &MutModule) -> Result<usize, diesel::result::Error> {
        let db = establish_database_connection();

        Ok(diesel::insert_or_ignore_into(modules::table).values(new_module).execute(&db)?)
    }

    fn read_one(mod_id: i32) -> Result<Self, diesel::result::Error> {
        use modules::dsl::module_id;
        let db = establish_database_connection();

        Ok(modules::table.filter(module_id.eq(mod_id)).first::<Self>(&db)?)
    }

    fn read_all() -> Result<Vec<Self>, diesel::result::Error> {
        let db = establish_database_connection();

        Ok(modules::table.load::<Module>(&db)?)
    }

    fn delete(mod_id: i32) -> Result<usize, diesel::result::Error> {
        use schema::modules::dsl::module_id;
        use schema::modules::dsl::modules;
        let db = establish_database_connection();

        Ok(diesel::delete(modules.filter(module_id.eq(mod_id))).execute(&db)?)
    }

    fn update(mod_id: i32, new_module: &MutModule) -> Result<usize, diesel::result::Error> {
        use schema::modules::dsl::module_id;
        use schema::modules::dsl::modules;
        let db = establish_database_connection();

        Ok(diesel::update(modules.filter(module_id.eq(mod_id))).set(new_module).execute(&db)?)
    }
}