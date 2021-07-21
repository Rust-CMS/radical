use diesel::prelude::*;
use diesel::{Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use super::page_models::Page;
use super::{Model};
use crate::schema::module_category;
use crate::schema::modules;

#[derive(Debug, Identifiable, Associations, Serialize, Deserialize, Queryable, PartialEq, Clone, Eq, Hash)]
#[belongs_to(Page, foreign_key = "page_uuid")]
#[belongs_to(ModuleCategory, foreign_key = "category_uuid")]
#[primary_key(uuid)]
#[table_name = "modules"]
pub struct Module {
    pub uuid: String,
    pub page_uuid: String,
    pub category_uuid: Option<String>,
    pub title: String,
    pub content: String,
}

#[derive(Insertable, AsChangeset, Deserialize, Serialize, Clone)]
#[table_name = "modules"]
pub struct MutModule {
    pub uuid: Option<String>,
    pub title: String,
    pub page_uuid: String,
    pub category_uuid: Option<String>,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryDTO {
    pub uuid: String,
    pub title: String,
    pub modules: Vec<Module>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FieldsDTO {
    pub modules: Vec<Module>,
    pub categories: Option<Vec<CategoryDTO>>
}

#[derive(
    Debug, Identifiable, Associations, Serialize, Deserialize, Queryable, PartialEq, Clone, Eq, Hash,
)]
#[primary_key(uuid)]
#[belongs_to(Page, foreign_key = "page_uuid")]
#[table_name = "module_category"]
pub struct ModuleCategory {
    pub uuid: String,
    pub page_uuid: String,
    pub title: String
}

#[derive(
    Debug, Serialize, Deserialize, AsChangeset, Insertable, PartialEq, Clone, Eq, Hash,
)]
#[table_name = "module_category"]
pub struct MutCategory {
    pub title: String,
    pub page_uuid: String,
    pub uuid: Option<String>
}

impl ModuleCategory {
    pub fn join(_id: String, db: &MysqlConnection) -> Result<Vec<Module>, diesel::result::Error> {
        use module_category::dsl::uuid;
        let categories = module_category::table.filter(uuid.eq(_id)).first::<Self>(db)?;

        Module::belonging_to(&categories).load::<Module>(db)
    }
}

impl Model<Self, MutCategory, String, ModuleCategory> for ModuleCategory {
    fn create(new: &MutCategory, db: &MysqlConnection) -> Result<usize, diesel::result::Error> {
        Ok(diesel::insert_or_ignore_into(module_category::table)
            .values(new)
            .execute(db)?)
    }

    fn read_one(_id: String, db: &MysqlConnection) -> Result<ModuleCategory, diesel::result::Error> {
        use module_category::dsl::uuid;

        let module = module_category::table.filter(uuid.eq(_id)).first::<ModuleCategory>(db)?;

        Ok(module)
    }

    fn read_all(_db: &MysqlConnection) -> Result<Vec<ModuleCategory>, diesel::result::Error> {
        unimplemented!()
    }

    fn update(
        _id: String,
        new: &MutCategory,
        db: &MysqlConnection,
    ) -> Result<usize, diesel::result::Error> {
        use module_category::dsl::uuid;

        Ok(diesel::update(module_category::table.filter(uuid.eq(_id)))
            .set(new)
            .execute(db)?)
    }

    fn delete(_id: String, db: &MysqlConnection) -> Result<usize, diesel::result::Error> {
        use module_category::dsl::uuid;

        Ok(diesel::delete(module_category::table.filter(uuid.eq(_id))).execute(db)?)
    }
}

impl Model<Self, MutModule, String, Module> for Module {
    fn create(
        new_module: &MutModule,
        db: &MysqlConnection,
    ) -> Result<usize, diesel::result::Error> {
        Ok(diesel::insert_into(modules::table)
            .values(new_module)
            .execute(db)?)
    }

    fn read_one(mod_id: String, db: &MysqlConnection) -> Result<Module, diesel::result::Error> {
        use modules::dsl::uuid;

        let module = modules::table.filter(uuid.eq(mod_id)).first::<Self>(db)?;

        Ok(module.into())
    }

    fn read_all(db: &MysqlConnection) -> Result<Vec<Module>, diesel::result::Error> {
        use modules::dsl::category_uuid;
        Ok(modules::table
            .filter(category_uuid.is_null())
            .load::<Module>(db)?.into_iter().map(|m| { m.into() }).collect())
    }

    fn delete(mod_id: String, db: &MysqlConnection) -> Result<usize, diesel::result::Error> {
        use modules::dsl::uuid;

        Ok(diesel::delete(modules::table.filter(uuid.eq(mod_id))).execute(db)?)
    }

    fn update(
        mod_id: String,
        new_module: &MutModule,
        db: &MysqlConnection,
    ) -> Result<usize, diesel::result::Error> {
        use modules::dsl::uuid;

        Ok(diesel::update(modules::table.filter(uuid.eq(mod_id)))
            .set(new_module)
            .execute(db)?)
    }
}