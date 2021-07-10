use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::module_models::Module;
use super::Model;
use crate::models::module_models::CategoryDTO;
use crate::models::module_models::ModuleCategory;
use crate::models::module_models::ModuleDTO;
use crate::schema::module_category;
use crate::schema::pages;

#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable, PartialEq, Clone)]
pub struct Page {
    pub id: i32,
    /// This should match the name of the HTML file.
    pub page_name: String,
    /// This should be the path which the program matches on.
    pub page_url: String,
    pub page_title: String,
    pub time_created: NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Deserialize, Serialize)]
#[table_name = "pages"]
pub struct MutPage {
    pub id: Option<i32>,
    pub page_name: String,
    pub page_url: String,
    pub page_title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageModuleDTO {
    pub page_id: i32,
    pub page_name: String,
    pub page_url: String,
    pub page_title: String,
    pub time_created: NaiveDateTime,
    /// the key of the hashmap is the `title` of the module, and the rest is the module.
    /// For the usefulness of this, see the `get` function on the default helpers.
    pub fields: HashMap<String, Module>,
    pub array_fields: HashMap<String, Vec<Module>>,
}

impl From<Page> for PageModuleDTO {
    fn from(origin_page: Page) -> Self {
        Self {
            page_name: origin_page.page_name.to_string(),
            page_url: origin_page.page_url.to_string(),
            page_title: origin_page.page_title.to_string(),
            time_created: origin_page.time_created,
            page_id: origin_page.id,
            fields: HashMap::new(),
            array_fields: HashMap::new(),
        }
    }
}

impl Model<Page, MutPage, i32> for Page {
    fn create(new_page: &MutPage, db: &MysqlConnection) -> Result<usize, diesel::result::Error> {
        Ok(diesel::insert_or_ignore_into(pages::table)
            .values(new_page)
            .execute(db)?)
    }

    fn read_one(_id: i32, db: &MysqlConnection) -> Result<Self, diesel::result::Error> {
        use crate::schema::pages::dsl::id;
        use crate::schema::pages::dsl::pages;

        pages.filter(id.eq(_id)).first::<Self>(db)
    }

    fn read_all(db: &MysqlConnection) -> Result<Vec<Self>, diesel::result::Error> {
        pages::table.load::<Self>(db)
    }

    fn update(
        _id: i32,
        new_page: &MutPage,
        db: &MysqlConnection,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::pages::dsl::id;
        use crate::schema::pages::dsl::pages;

        Ok(diesel::update(pages.filter(id.eq(_id)))
            .set(new_page)
            .execute(db)?)
    }

    fn delete(_id: i32, db: &MysqlConnection) -> Result<usize, diesel::result::Error> {
        use crate::schema::pages::dsl::id;
        use crate::schema::pages::dsl::pages;

        Ok(diesel::delete(pages.filter(id.eq(_id))).execute(db)?)
    }
}

impl Page {
    pub fn read_one_join_on(
        id: String,
        db: &MysqlConnection,
    ) -> Result<(Self, ModuleDTO), diesel::result::Error> {
        use crate::schema::pages::dsl::page_url;

        let filtered_page = pages::table.filter(page_url.eq(id)).first::<Page>(db)?;

        let modules = Module::belonging_to(&filtered_page).load::<Module>(db)?;

        let categories = Module::belonging_to(&filtered_page)
            .inner_join(module_category::table)
            .select(module_category::all_columns)
            .load::<ModuleCategory>(db)?;

        let module_array: Vec<(Vec<Module>, ModuleCategory)> = Module::belonging_to(&categories)
            .load::<Module>(db)?
            .grouped_by(&categories)
            .iter()
            .map(|a| a.clone())
            .zip(categories)
            .collect::<Vec<_>>();

        let category_dtos: Vec<CategoryDTO> = module_array
            .iter()
            .map(|a| CategoryDTO {
                id: a.1.id,
                title: a.1.title.clone(),
                modules: a.0.clone(),
            })
            .collect::<Vec<_>>();

        let module_dto: ModuleDTO = ModuleDTO {
            modules: modules,
            categories: Some(category_dtos),
        };

        Ok((filtered_page, module_dto))
    }
}
