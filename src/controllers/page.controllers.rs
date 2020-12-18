use actix_web::{HttpResponse, Responder};

pub async fn root() -> impl Responder {
    HttpResponse::Ok().body("unimplemented")
}

pub async fn pages() -> impl Responder {
    HttpResponse::Ok().body("unimplemented")
}

pub async fn page() -> impl Responder {
    HttpResponse::Ok().body("unimplemented")
}