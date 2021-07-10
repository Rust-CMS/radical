use std::sync::Mutex;

use actix_web::{web, HttpResponse};
use handlebars::Handlebars;

use crate::models::{pool_handler, Model, MySQLPool};

use crate::models::module_models::{ModuleDTO};
use crate::models::page_models::PageModuleDTO;
use crate::models::page_models::{MutPage, Page};

use crate::middleware::errors_middleware::CustomHttpError;
use crate::middleware::response_middleware::HttpResponseBuilder;

fn parse_page(page: (Page, ModuleDTO)) -> Result<PageModuleDTO, CustomHttpError> {
    let origin_page = page.0;

    // cast the origin page that is always standard into a new object that has the modules as a vec of children.
    let mut res: PageModuleDTO = origin_page.into();

    match page.1.categories {
        Some(modules) => {
            for module in modules {
                res.array_fields.insert(module.title, module.modules);
            }
        },
        None => {}
    };

    for module in page.1.modules {
        res.fields.insert(module.title.clone(), module);
    }

    Ok(res)
}

pub async fn display_page(
    req: web::HttpRequest,
    pool: web::Data<MySQLPool>,
    hb: web::Data<Mutex<Handlebars<'_>>>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let path = req.path();
    let page_tuple = Page::read_one_join_on(path.to_string(), &mysql_pool);

    if let Err(_) = page_tuple {
        let s = hb.lock().unwrap().render("404", &String::from("")).unwrap();
        return Ok(HttpResponse::Ok().content_type("text/html").body(s));
    }

    let pagemodule = parse_page(page_tuple?)?;

    let s = hb
        .lock()
        .unwrap()
        .render(&pagemodule.page_name, &pagemodule)
        .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn create_page(
    new_page: web::Json<MutPage>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    Page::create(&new_page, &mysql_pool)?;

    HttpResponseBuilder::new(201, &*new_page)
}

pub async fn get_pages(pool: web::Data<MySQLPool>) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let pages: Vec<Page> = Page::read_all(&mysql_pool)?;

    HttpResponseBuilder::new(200, &pages)
}

pub async fn get_page(
    id: web::Path<i32>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let page: Page = Page::read_one(*id, &mysql_pool)?;

    HttpResponseBuilder::new(200, &page)
}

/// This function also parses the read_one_join_on result out of a tuple.
/// A tuple generates a nasty response that isn't well defined.
/// This function parses it in to a Page that has all of the Modules as children.
pub async fn get_page_join_modules(
    id: web::Path<i32>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let page_vec = Page::read_one_join_on(id.to_string(), &mysql_pool)?;

    let pagemodules = parse_page(page_vec).or(Err(CustomHttpError::NotFound))?;

    HttpResponseBuilder::new(200, &pagemodules)
}

pub async fn update_page(
    updated_page: web::Json<MutPage>,
    id: web::Path<i32>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    Page::update(*id, &updated_page, &mysql_pool)?;

    HttpResponseBuilder::new(200, &*updated_page)
}

pub async fn delete_page(
    id: web::Path<i32>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    Page::delete(*id, &mysql_pool)?;

    HttpResponseBuilder::new(200, &format!("Successfully deleted resource {}", id))
}
