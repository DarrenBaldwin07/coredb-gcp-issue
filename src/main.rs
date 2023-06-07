use rapid_web::server::RapidServer;
use rapid_web::actix::{web, HttpServer};
use rapid_web::rapid_web_codegen::{main, routes, rapid_configure};
pub mod schema;
use r2d2_postgres::{PostgresConnectionManager};
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use diesel::{
	r2d2::Pool
};

rapid_configure!("src/routes");

pub type DatabaseConnection = Pool<PostgresConnectionManager<MakeTlsConnector>>;

pub struct Database {
	pub url: String,
}

impl Database {
	pub fn connection_pool(&self) -> DatabaseConnection {
		let connector: TlsConnector = TlsConnector::builder().danger_accept_invalid_certs(true).build().unwrap();
		let connector = MakeTlsConnector::new(connector);
		let manager = PostgresConnectionManager::new(
			self.url.parse().unwrap(),
			connector,
		);
		diesel::r2d2::Pool::builder().build_unchecked(manager)
	}
}


/// USE THIS CODE TO TEST TLS/SSL ISSUE WITH COREDB:
// use diesel::{
// 	pg::PgConnection,
// 	r2d2::{ConnectionManager, Pool},
// };

// pub type DatabaseConnection = Pool<ConnectionManager<PgConnection>>;

// pub struct Database {
// 	pub url: String,
// }

// impl Database {
// 	pub fn connection_pool(&self) -> DatabaseConnection {
// 		let manager = ConnectionManager::<PgConnection>::new(&self.url);
// 		// Refer to the `r2d2` documentation for more methods to use
// 		// when building this connection pool (https://crates.io/crates/r2d2)
// 		Pool::builder()
// 			.test_on_check_out(true)
// 			.build(manager)
// 			.expect("Could not build connection pool")
// 	}
// }



#[main]
async fn main() -> std::io::Result<()> {
    let app = RapidServer::create(None, Some(String::from("0.0.0.0")));
    let db = Database {
        url: String::from("postgresql://postgres:P0bjjocmgaIhJ28Y@org-cincinnati-ventures-inst-job-jar-staging.data-1.use1.coredb.io:5432")
    };

    db.connection_pool().get().expect("Could not get connection pool!");

    app.listen(HttpServer::new(move || {
        RapidServer::fs_router(None, None, routes!("src/routes"))
    })).await
}
