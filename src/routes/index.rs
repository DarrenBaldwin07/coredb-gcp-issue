use rapid_web::actix::HttpResponse;
use rapid_web::{rapid_web_codegen::rapid_handler, welcome_view};
use diesel::{
	pg::PgConnection,
	r2d2::{ConnectionManager, Pool},
};
use diesel::prelude::*;
use sqlx::{PgPool, postgres::PgPoolOptions};
use crate::schema::posts::dsl::*;



pub const ROUTE_KEY: &str = "index";


pub type DatabaseConnection = Pool<ConnectionManager<PgConnection>>;

pub struct Database {
	pub url: String,
}

impl Database {
	pub fn connection_pool(&self) -> DatabaseConnection {
		let manager = ConnectionManager::<PgConnection>::new(&self.url);

		Pool::builder()
			.build(manager)
			.expect("Could not build connection pool")
	}
}


#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[rapid_handler]
pub async fn query() -> HttpResponse {
	let db = Database {
        url: String::from("postgres://postgres:ossToTheMoon747388945@db.gvkicyipudcascybkfrj.supabase.co:6543/postgres")
    };

    db.connection_pool();

    HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(welcome_view)
}
