use diesel::Connection;
use rapid_web::server::RapidServer;
use rapid_web::actix::{web, HttpServer};
use rapid_web::rapid_web_codegen::{main, routes, rapid_configure};

rapid_configure!("src/routes");

use diesel::{
	pg::PgConnection,
	r2d2::{ConnectionManager, Pool},
};

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
			.test_on_check_out(true)
			.build(manager)
			.expect("Could not build connection pool")
	}
}


#[main]
async fn main() -> std::io::Result<()> {
    let app = RapidServer::create(None, None);

    let db = Database {
        url: String::from("postgresql://postgres:z5t1m5rrDRVZ23bs@org-cincinnati-ventures-inst-job-jar-prod.data-1.use1.coredb.io:5432")
    };

    db.connection_pool();

    app.listen(HttpServer::new(move || {
        RapidServer::fs_router(None, None, routes!("src/routes"))
    })).await
}
