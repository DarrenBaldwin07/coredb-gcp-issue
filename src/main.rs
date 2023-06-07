use diesel::{PgConnection, r2d2::ConnectionManager};
use diesel::prelude::*;
use crate::schema::{posts, posts::dsl::*};
pub mod schema;

use diesel::{
	r2d2::Pool
};

use std::env;

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

#[derive(Insertable, AsChangeset)]
#[table_name = "posts"]

pub struct NewPost<'a> {
	pub title: &'a str,
	pub body: &'a str,
}


pub fn establish_connection() -> PgConnection {

    let database_url = String::from("postgresql://postgres:********@org-cincinnati-ventures-inst-job-jar-staging.data-1.use1.coredb.io:5432/postgres?sslmode=require");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


fn main() {

	establish_connection();
}
