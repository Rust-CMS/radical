use diesel::{Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[path = "../schemas/schema.rs"]
mod schema;

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

impl Model<Module, MutModule> for Module {
    fn create(new_module: &MutModule) {
        let db = establish_database_connection();

        diesel::insert_into(modules::table).values(new_module).execute(&db).unwrap();
    }

    fn read_one(mod_id: i32) -> Module {
        use modules::dsl::module_id;
        let db = establish_database_connection();

        modules::table.filter(module_id.eq(mod_id)).first::<Self>(&db).unwrap()
    }

    fn read_all() -> Vec<Module> {
        let db = establish_database_connection();

        modules::table.load::<Module>(&db).unwrap()
    }

    fn delete(mod_id: i32) {
        use schema::modules::dsl::module_id;
        use schema::modules::dsl::modules;

        let db = establish_database_connection();

        diesel::delete(modules.filter(module_id.eq(mod_id))).execute(&db).unwrap();
    }

    fn update(mod_id: i32, new_module: &MutModule) {
        use schema::modules::dsl::module_id;
        use schema::modules::dsl::modules;
        let db = establish_database_connection();

        diesel::update(modules.filter(module_id.eq(mod_id))).set(new_module).execute(&db).unwrap();
    }
}