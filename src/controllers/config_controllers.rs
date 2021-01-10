use actix_web::{web, HttpRequest, HttpResponse};
use config_models::{Config, MutConfig};
use serde::{Deserialize, Serialize};

use std::{
    fs::{File, OpenOptions},
    io::BufReader,
};

use models::Model;

use crate::{
    config_models,
    errors_middleware::{map_int_parsing_error, map_sql_error, CustomHttpError},
    models::{self, pool_handler, MySQLPool},
    response_middleware::HttpResponseBuilder,
};

/// This will be exported into by serde. See the `rcms.json` file in the root of the project for config information.
#[derive(Deserialize, Serialize)]
pub struct LocalConfig {
    pub mysql_username: Option<String>,
    pub mysql_password: Option<String>,
    pub mysql_database: Option<String>,
    pub mysql_url: Option<String>,
    pub mysql_port: Option<u16>,
}

pub async fn update_local_config(
    conf: web::Json<LocalConfig>,
) -> Result<HttpResponse, CustomHttpError> {
    let config_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("./rcms.json")
        .or(Err(CustomHttpError::Unknown))?;
    serde_json::to_writer(config_file, &conf.0).or(Err(CustomHttpError::Unknown))?;

    HttpResponseBuilder::new(200, &"Unimplemented.")
}

pub async fn read_all_local_config() -> Result<HttpResponse, CustomHttpError> {
    let config_file = File::open("./rcms.json").or(Err(CustomHttpError::Unknown))?;
    let reader = BufReader::new(config_file);
    let conf: LocalConfig = serde_json::from_reader(reader).or(Err(CustomHttpError::Unknown))?;

    HttpResponseBuilder::new(200, &conf)
}

pub async fn read_all_database_config(
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let all = Config::read_all(&mysql_pool).map_err(map_sql_error)?;
    HttpResponseBuilder::new(200, &all)
}

pub async fn read_one_database_config(
    req: HttpRequest,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let config_id: String = req
        .match_info()
        .get("id")
        .ok_or(CustomHttpError::BadRequest)?
        .to_owned();
    let one = Config::read_one(config_id, &mysql_pool).map_err(map_sql_error)?;
    HttpResponseBuilder::new(200, &one)
}

pub async fn update_database_config(
    req: HttpRequest,
    pool: web::Data<MySQLPool>,
    mut_config: web::Json<MutConfig>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let config_id: String = req
        .match_info()
        .get("id")
        .ok_or(CustomHttpError::BadRequest)?
        .to_owned();
    let us = Config::update(config_id, &mut_config, &mysql_pool).map_err(map_sql_error)?;
    HttpResponseBuilder::new(200, &us)
}
