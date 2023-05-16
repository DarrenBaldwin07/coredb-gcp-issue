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
	// THIS WORKS:
	PgPoolOptions::new()
    .max_connections(100)
    .connect("postgresql://postgres:d8MEnLYKL6p5SeqG@org-cincinnati-ventures-inst-test-db.data-1.use1.coredb.io:5432")
    .await
    .expect("Failed to create pool.");


	// THIS DOES NOT WORK (it throws an error via .expect())
	let db = Database {
        url: String::from("postgresql://postgres:d8MEnLYKL6p5SeqG@org-cincinnati-ventures-inst-test-db.data-1.use1.coredb.io:5432")
    };

    let connection = db.connection_pool();

    HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(welcome_view)
}
