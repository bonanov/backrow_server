use diesel::prelude::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

mod errors;
mod users;

pub use errors::*;
pub use users::*;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub fn get_pool() -> DbPool {
    dotenv().ok();
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
