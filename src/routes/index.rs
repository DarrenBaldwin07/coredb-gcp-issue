use rapid_web::actix::HttpResponse;
use rapid_web::{rapid_web_codegen::rapid_handler, welcome_view};
use diesel::{
	r2d2::Pool
};
use diesel::prelude::*;

pub const ROUTE_KEY: &str = "index";


#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[rapid_handler]
pub async fn query() -> HttpResponse {
    HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(welcome_view)
}
