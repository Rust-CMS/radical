use diesel::prelude::*;
use diesel::{Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use super::page_models::Page;
use super::{DTO, Model};
use crate::schema::module_category;
use crate::schema::modules;

#[derive(
    Debug, Identifiable, Associations, Serialize, Deserialize, Queryable, PartialEq, Clone, Eq, Hash,
)]
#[belongs_to(Page)]
#[belongs_to(ModuleCategory, foreign_key = "category")]
#[table_name = "modules"]
pub struct Module {
    pub id: i32,
    pub uuid: String,
    pub module_type_id: i32,
    pub title: String,
    pub page_id: i32,
    pub page_uuid: String,
    pub content: String,
    pub category: Option<i32>,
}

#[derive(Insertable, AsChangeset, Deserialize, Serialize)]
#[table_name = "modules"]
pub struct MutModule {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub module_type_id: i32,
    pub title: String,
    pub page_id: i32,
    pub content: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct ModuleCategoryDTO {
    pub uuid: String,
    pub title: String,
}

impl From<ModuleCategory> for ModuleCategoryDTO {
    fn from(category: ModuleCategory) -> ModuleCategoryDTO {
        ModuleCategoryDTO {
            uuid: category.uuid,
            title: category.title,
        }
    }
}

type ModuleCategoryColumns = (module_category::columns::uuid, module_category::columns::title);
impl DTO<ModuleCategoryColumns> for ModuleCategoryDTO {
    fn columns() -> ModuleCategoryColumns {
        (module_category::columns::uuid, module_category::columns::title)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryDTO {
    pub uuid: String,
    pub title: String,
    pub modules: Vec<ModuleDTO>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FieldsDTO {
    pub modules: Vec<ModuleDTO>,
    pub categories: Option<Vec<CategoryDTO>>
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable)]
pub struct ModuleDTO {
    pub uuid: String,
    pub title: String,
    pub content: String,
    pub page_uuid: String
}

impl From<Module> for ModuleDTO {
    fn from(module: Module) -> Self {
        Self {
            uuid: module.uuid,
            title: module.title,
            content: module.content,
            page_uuid: module.page_uuid
        }
    }
}

type ModuleDTOColumns = (modules::columns::uuid, modules::columns::title, modules::columns::content, modules::columns::page_uuid);
impl DTO<ModuleDTOColumns> for ModuleDTO {
    fn columns() -> ModuleDTOColumns {
        use modules::columns::*;
        (uuid, title, content, page_uuid)
    }
}

#[derive(
    Debug, Identifiable, Associations, Serialize, Deserialize, Queryable, PartialEq, Clone, Eq, Hash,
)]
#[table_name = "module_category"]
pub struct ModuleCategory {
    pub id: i32,
    pub uuid: String,
    pub title: String
}

#[derive(
    Debug, Serialize, Deserialize, AsChangeset, Insertable, PartialEq, Clone, Eq, Hash,
)]
#[table_name = "module_category"]
pub struct MutCategory {
    pub title: String
}

impl ModuleCategory {
    pub fn join(_id: String, db: &MysqlConnection) -> Result<Vec<ModuleDTO>, diesel::result::Error> {
        use module_category::dsl::uuid;
        let categories = module_category::table.filter(uuid.eq(_id)).first::<Self>(db)?;

        Module::belonging_to(&categories).select(ModuleDTO::columns()).load::<ModuleDTO>(db)
    }
}

impl Model<Self, MutCategory, String, ModuleCategoryDTO> for ModuleCategory {
    fn create(new: &MutCategory, db: &MysqlConnection) -> Result<usize, diesel::result::Error> {
        Ok(diesel::insert_or_ignore_into(module_category::table)
            .values(new)
            .execute(db)?)
    }

    fn read_one(_id: String, db: &MysqlConnection) -> Result<ModuleCategoryDTO, diesel::result::Error> {
        use module_category::dsl::uuid;

        let module = module_category::table.select(ModuleCategoryDTO::columns()).filter(uuid.eq(_id)).first::<ModuleCategoryDTO>(db)?;

        Ok(module)
    }

    fn read_all(_db: &MysqlConnection) -> Result<Vec<ModuleCategoryDTO>, diesel::result::Error> {
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

impl Model<Self, MutModule, String, ModuleDTO> for Module {
    fn create(
        new_module: &MutModule,
        db: &MysqlConnection,
    ) -> Result<usize, diesel::result::Error> {
        Ok(diesel::insert_or_ignore_into(modules::table)
            .values(new_module)
            .execute(db)?)
    }

    fn read_one(mod_id: String, db: &MysqlConnection) -> Result<ModuleDTO, diesel::result::Error> {
        use modules::dsl::uuid;

        let module = modules::table.filter(uuid.eq(mod_id)).first::<Self>(db)?;

        Ok(module.into())
    }

    fn read_all(db: &MysqlConnection) -> Result<Vec<ModuleDTO>, diesel::result::Error> {
        use modules::dsl::category;
        Ok(modules::table
            .filter(category.is_null())
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