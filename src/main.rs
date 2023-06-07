use diesel::{PgConnection, r2d2::ConnectionManager};
pub mod schema;
use diesel::{
	r2d2::Pool
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



fn main() {
    let db = Database {
        url: String::from("postgresql://postgres:P0bjjocmgaIhJ28Y@org-cincinnati-ventures-inst-job-jar-staging.data-1.use1.coredb.io:5432")
    };

    db.connection_pool().get().expect("Could not get connection pool!");
}
