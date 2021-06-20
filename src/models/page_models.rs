use std::collections::HashMap;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use super::Joinable;

use super::Model;
use super::module_models::Module;
use crate::schema::pages;

/// The main Rust implementation for the Page model.
#[derive(Debug, Serialize, Deserialize, Queryable, PartialEq, Clone)]
pub struct Page {
    pub id: i32,
    /// This should match the name of the HTML file.
    pub page_name: String,
    /// This should be the path which the program matches on.
    pub page_url: String,
    pub page_title: String,
    pub time_created: NaiveDateTime,
}
/// This acts as both the insertable and update object.
/// This can be done since pages only really have a `title` column that isn't auto filled.
#[derive(Insertable, AsChangeset, Deserialize, Serialize)]
#[table_name = "pages"]
pub struct MutPage {
    pub page_url: String,
    pub page_title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageModuleRelation {
    pub page_id: i32,
    pub page_name: String,
    pub page_url: String,
    pub page_title: String,
    pub time_created: NaiveDateTime,
    /// the key of the hashmap is the `title` of the module, and the rest is the module.
    pub fields: HashMap<String, Module>,
}

impl Model<Page, MutPage, i32> for Page {
    fn create(new_page: &MutPage, db: &MysqlConnection) -> Result<usize, diesel::result::Error> {
        Ok(diesel::insert_or_ignore_into(pages::table)
            .values(new_page)
            .execute(db)?)
    }

    fn read_one(_id: i32, db: &MysqlConnection) -> Result<Self, diesel::result::Error> {
        use crate::schema::pages::dsl::pages;
        use crate::schema::pages::dsl::id;

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
        use crate::schema::pages::dsl::pages;
        use crate::schema::pages::dsl::id;

        Ok(diesel::update(pages.filter(id.eq(_id)))
            .set(new_page)
            .execute(db)?)
    }

    fn delete(_id: i32, db: &MysqlConnection) -> Result<usize, diesel::result::Error> {
        use crate::schema::pages::dsl::pages;
        use crate::schema::pages::dsl::id;

        Ok(diesel::delete(pages.filter(id.eq(_id))).execute(db)?)
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
        use crate::schema::pages::dsl::page_url;

        pages
            .inner_join(modules)
            .filter(page_url.eq(id))
            .load::<(Page, Module)>(db)
    }
}
