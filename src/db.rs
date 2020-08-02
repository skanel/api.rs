use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel::connectionManager;

use rocket::http::Status;
use rocket::request::{self, FromRequest};

use rocket::request::{Outcome, Request, State};

pub fn init_pool(db_url: String) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::new(manager).expect("db pool failure")
}

