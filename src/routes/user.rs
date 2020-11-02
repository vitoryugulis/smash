use actix_web::{HttpResponse, Responder, get, post, web};
use mysql::Pool;
use crate::models::{person::Person};
use crate::controllers::person;

#[post("/user")]
pub async fn post(
    data: web::Json<Person>,
    pool: web::Data<Pool>
) -> impl Responder {
    let result = person::create(data.clone(), &pool.clone()).await;
    match result {
        Ok(r) => return HttpResponse::Ok().body(r),
        Err(e) => return HttpResponse::InternalServerError().body(e.error)
    }
}

#[get("/user")]
pub async fn get(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
