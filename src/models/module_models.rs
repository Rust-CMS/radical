use diesel::prelude::*;
use diesel::{Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use super::page_models::Page;
use super::Model;
use crate::schema::module_category;
use crate::schema::modules;

#[derive(
    Debug, Identifiable, Associations, Serialize, Deserialize, Queryable, PartialEq, Clone, Eq, Hash,
)]
#[belongs_to(Page)]
#[belongs_to(ModuleCategory, foreign_key = "category")]
#[primary_key(module_id)]
#[table_name = "modules"]
pub struct Module {
    pub module_id: i32,
    pub module_type_id: i32,
    pub title: String,
    pub page_id: i32,
    pub content: String,
    pub category: Option<i32>,
}

#[derive(Insertable, AsChangeset, Deserialize, Serialize)]
#[table_name = "modules"]
pub struct MutModule {
    pub module_id: Option<i32>,
    pub module_type_id: i32,
    pub title: String,
    pub page_id: i32,
    pub content: String,
}

#[derive(
    Debug, Identifiable, Associations, Serialize, Deserialize, Queryable, PartialEq, Clone, Eq, Hash,
)]
#[table_name = "module_category"]
pub struct ModuleCategory {
    pub id: i32,
    pub title: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryDTO {
    pub id: i32,
    pub title: String,
    pub modules: Vec<Module>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ModuleDTO {
    pub modules: Vec<Module>,
    pub categories: Option<Vec<CategoryDTO>>
}

impl Model<Self, MutModule, i32> for Module {
    fn create(
        new_module: &MutModule,
        db: &MysqlConnection,
    ) -> Result<usize, diesel::result::Error> {
        Ok(diesel::insert_or_ignore_into(modules::table)
            .values(new_module)
            .execute(db)?)
    }

    fn read_one(mod_id: i32, db: &MysqlConnection) -> Result<Module, diesel::result::Error> {
        use modules::dsl::module_id;

        let module = modules::table.filter(module_id.eq(mod_id)).first::<Self>(db)?;

        Ok(module)
    }

    fn read_all(db: &MysqlConnection) -> Result<Vec<Module>, diesel::result::Error> {
        use modules::dsl::category;
        Ok(modules::table
            .filter(category.is_null())
            .load::<Module>(db)?)
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