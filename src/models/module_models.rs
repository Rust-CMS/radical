use diesel::{Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[path = "../schemas/schema.rs"]
mod schema;

use crate::page_models::Page;

use super::models::{Model, establish_database_connection};

use schema::modules;

#[derive(Debug, Serialize, Deserialize, Queryable, PartialEq)]
pub struct Module {
    module_id: i32,
    module_type_id: i32,
    page_id: i32,
    content: Option<String>
}

#[derive(Insertable, AsChangeset, Deserialize, Serialize)]
#[table_name = "modules"]
pub struct MutModule {
    module_type_id: i32,
    page_id: i32,
    content: Option<String>
}

impl Model<Module, MutModule, Page> for Module {
    fn create(new_module: &MutModule) -> Result<usize, diesel::result::Error> {
        let db = establish_database_connection();

        Ok(diesel::insert_into(modules::table).values(new_module).execute(&db)?)
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

    fn read_one_join_on(id: i32) -> Result<Vec<(Self, Page)>, diesel::result::Error> {
        use schema::modules::dsl::module_id;
        use schema::modules::dsl::modules;
        use schema::pages::dsl::pages;
        let db = establish_database_connection();

        modules.inner_join(pages).filter(module_id.eq(id)).load::<(Module, Page)>(&db)
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