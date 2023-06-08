use diesel::{PgConnection, r2d2::ConnectionManager};
use diesel::prelude::*;
use crate::schema::{posts, posts::dsl::*};
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

#[derive(Insertable, AsChangeset)]
#[table_name = "posts"]

pub struct NewPost<'a> {
	pub title: &'a str,
	pub body: &'a str,
}

fn main() {
    let db = Database {
        url: String::from("postgresql://postgres:4Ll4BzezlNTL8ujU@org-cincinnati-ventures-inst-rezon.data-1.use1.coredb.io:5432")
    };

    let mut pool = db.connection_pool().get().expect("Could not get connection pool!");

	let new_company_creation: NewPost = NewPost {
		title: "thing2222",
		body: "dafa2222",
	};

	diesel::insert_into(posts).values(&new_company_creation).execute(&mut pool);

}
