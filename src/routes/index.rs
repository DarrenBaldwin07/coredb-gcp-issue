use rapid_web::actix::HttpResponse;
use rapid_web::{rapid_web_codegen::rapid_handler, welcome_view};
use diesel::{
	pg::PgConnection,
	r2d2::{ConnectionManager, Pool},
};
use sqlx::{PgPool, postgres::PgPoolOptions};

pub const ROUTE_KEY: &str = "index";


pub type DatabaseConnection = Pool<ConnectionManager<PgConnection>>;

pub struct Database {
	pub url: String,
}

impl Database {
	pub fn connection_pool(&self) -> DatabaseConnection {
		let manager = ConnectionManager::<PgConnection>::new(&self.url);
		// Refer to the `r2d2` documentation for more methods to use
		// when building this connection pool (https://crates.io/crates/r2d2)
		Pool::builder()
			.min_idle(Some(1))
            .max_size(1)
			.build(manager)
			.expect("Could not build connection pool")
	}
}

#[rapid_handler]
pub async fn query() -> HttpResponse {
	PgPoolOptions::new()
    .max_connections(100)
    .connect("postgresql://postgres:z5t1m5rrDRVZ23bs@org-cincinnati-ventures-inst-job-jar-prod.data-1.use1.coredb.io:5432")
    .await
    .expect("Failed to create pool.");

    HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(welcome_view)
}
