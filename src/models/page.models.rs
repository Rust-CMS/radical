use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[path = "../schemas/schema.rs"]
mod schema;

use schema::pages;
use super::models::{Model, establish_database_connection};

/// The main Rust implementation for the Page model.
#[derive(Debug, Serialize, Deserialize, Queryable, PartialEq)]
pub struct Page {
    page_id: i32,
    title: String,
    time_created: NaiveDateTime,
}
/// This acts as both the insertable and update object.
/// This can be done since pages only really have a `title` column that isn't auto filled.
#[derive(Insertable, AsChangeset, Deserialize, Serialize)]
#[table_name = "pages"]
pub struct MutPage {
    title: String,
}

/// Implementation for Page restricted by models.rs trait.
/// schema::...::dsl exports all of the columns.
/// It also exports the table name again. This allows for filtering through the rows of the table.
/// Every one of these functions exports only what they need out of `dsl`.
/// Taking all of the columns (for instance whenever using schema::pages::dsl::*) 
/// is unnecessary and leads to higher RAM usage.
impl Model<Page, MutPage> for Page {
    fn create(new_page: &MutPage) {
        let db = establish_database_connection();

        diesel::insert_into(pages::table).values(new_page).execute(&db).unwrap();
    }

    fn read_one(id: i32) -> Self {
        use schema::pages::dsl::pages;
        use schema::pages::dsl::page_id;
        
        let db = establish_database_connection();

        pages.filter(page_id.eq(id)).first::<Self>(&db).unwrap()
    }

    fn read_all() -> Vec<Self> {
        let db = establish_database_connection();

        pages::table.load::<Self>(&db).unwrap()
    }

    fn update(id: i32, new_page: &MutPage) {
        use schema::pages::dsl::page_id;
        use schema::pages::dsl::pages;

        let db = establish_database_connection();

        diesel::update(pages.filter(page_id.eq(id))).set(new_page).execute(&db).unwrap();

    }

    fn delete(id: i32) {
        use schema::pages::dsl::page_id;
        use schema::pages::dsl::pages;

        let db = establish_database_connection();

        diesel::delete(pages.filter(page_id.eq(id))).execute(&db).unwrap();
    }
}
