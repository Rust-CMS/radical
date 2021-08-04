use std::sync::Mutex;

use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use uuid::Uuid;

use crate::models::{pool_handler, Model, MySQLPool};

use crate::models::module_models::{FieldsDTO};
use crate::models::page_models::{PageModuleDisplayDTO,MutPage, Page, PageDTO};

use crate::services::auth_service::Claims;
use crate::services::errors_service::CustomHttpError;

fn parse_page(page: (Page, FieldsDTO)) -> Result<PageModuleDisplayDTO, CustomHttpError> {
    let origin_page = page.0;

    // cast the origin page that is always standard into a new object that has the modules as a vec of children.
    let mut res: PageModuleDisplayDTO = origin_page.into();

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
    let page_tuple = Page::read_one_join_on_url(path.to_string(), &mysql_pool);

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
    new: web::Json<MutPage>,
    pool: web::Data<MySQLPool>,
    _: Claims
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let mut uuid_new = new.clone();
    uuid_new.uuid = Some(Uuid::new_v4().to_string());

    Page::create(&uuid_new, &mysql_pool)?;

    Ok(HttpResponse::Ok().json(uuid_new))
}

pub async fn get_pages(pool: web::Data<MySQLPool>) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let pages: Vec<PageDTO> = Page::read_all(&mysql_pool)?;

    Ok(HttpResponse::Ok().json(pages))

}

pub async fn get_page(
    id: web::Path<String>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let page: PageDTO = Page::read_one(id.clone(), &mysql_pool)?;
    Ok(HttpResponse::Ok().json(page))

}

pub async fn get_page_join_modules(
    id: web::Path<String>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let page_vec = Page::read_one_join_on(id.clone(), &mysql_pool)?;

    Ok(HttpResponse::Ok().json(page_vec))
}

pub async fn update_page(
    updated_page: web::Json<MutPage>,
    id: web::Path<String>,
    pool: web::Data<MySQLPool>,
    _: Claims
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    Page::update(id.clone(), &updated_page, &mysql_pool)?;

    Ok(HttpResponse::Ok().json(updated_page.0))

}

pub async fn delete_page(
    id: web::Path<String>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let res = Page::delete(id.clone(), &mysql_pool)?;

    Ok(HttpResponse::Ok().json(res))
}
