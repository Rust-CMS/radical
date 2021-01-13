use std::collections::HashMap;

use askama::Template;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::{models::Joinable, module_models::Module};

use super::models::Model;
use crate::schema::pages;

/// The main Rust implementation for the Page model.
#[derive(Debug, Serialize, Deserialize, Queryable, PartialEq, Clone)]
pub struct Page {
    pub url_path: String,
    pub title: String,
    pub time_created: NaiveDateTime,
}
/// This acts as both the insertable and update object.
/// This can be done since pages only really have a `title` column that isn't auto filled.
#[derive(Insertable, AsChangeset, Deserialize, Serialize)]
#[table_name = "pages"]
pub struct MutPage {
    pub url_path: String,
    pub title: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ModuleHash {
    pub v: HashMap<String, Module>,
}

#[derive(Debug, Serialize, Clone, Template)]
#[template(path = "index.html")]
pub struct PageModuleRelation {
    pub url_path: String,
    pub title: String,
    pub time_created: NaiveDateTime,
    /// the key of the hashmap is the `title` of the module, and the rest is the module.
    pub fields: ModuleHash,
}

/// Implementation for Page restricted by models.rs trait.
/// schema::...::dsl exports all of the columns.
/// It also exports the table name again. This allows for filtering through the rows of the table.
/// Every one of these functions exports only what they need out of `dsl`.
/// Taking all of the columns (for instance whenever using schema::pages::dsl::*)
/// is unnecessary and leads to higher RAM usage.
impl Model<Page, MutPage, String> for Page {
    fn create(new_page: &MutPage, db: &MysqlConnection) -> Result<usize, diesel::result::Error> {
        Ok(diesel::insert_or_ignore_into(pages::table)
            .values(new_page)
            .execute(db)?)
    }

    fn read_one(id: String, db: &MysqlConnection) -> Result<Self, diesel::result::Error> {
        use crate::schema::pages::dsl::pages;
        use crate::schema::pages::dsl::url_path;

        pages.filter(url_path.eq(id)).first::<Self>(db)
    }

    fn read_all(db: &MysqlConnection) -> Result<Vec<Self>, diesel::result::Error> {
        pages::table.load::<Self>(db)
    }

    fn update(
        id: String,
        new_page: &MutPage,
        db: &MysqlConnection,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::pages::dsl::pages;
        use crate::schema::pages::dsl::url_path;

        Ok(diesel::update(pages.filter(url_path.eq(id)))
            .set(new_page)
            .execute(db)?)
    }

    fn delete(id: String, db: &MysqlConnection) -> Result<usize, diesel::result::Error> {
        use crate::schema::pages::dsl::pages;
        use crate::schema::pages::dsl::url_path;

        Ok(diesel::delete(pages.filter(url_path.eq(id))).execute(db)?)
    }
}

/// Separate implementation for joinable trait.
impl Joinable<Page, Module, String> for Page {
    fn read_one_join_on(
        id: String,
        db: &MysqlConnection,
    ) -> Result<Vec<(Self, Module)>, diesel::result::Error> {
        use crate::schema::modules::dsl::modules;
        use crate::schema::pages::dsl::pages;
        use crate::schema::pages::dsl::url_path;

        pages
            .inner_join(modules)
            .filter(url_path.eq(id))
            .load::<(Page, Module)>(db)
    }
}

impl ModuleHash {
    pub fn new() -> ModuleHash {
        ModuleHash { v: HashMap::new() }
    }

    pub fn get_field(&self, s: &str) -> String {
        self.v.get(s).unwrap().content.clone()
    }
}
