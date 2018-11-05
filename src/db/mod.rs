use actix::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub mod thing;

// This is db executor actor. We are going to run several of them in parallel.
pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    pub fn get_conn() -> PgConnection {
        dotenv::dotenv().ok();
        let dburl = env::var("DATABASE_URL").unwrap();
        diesel::pg::PgConnection::establish(&dburl).unwrap()
    }
}
