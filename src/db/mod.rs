use actix::prelude::*;
use actix_web::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use schema;

pub mod thing;

// This is db executor actor. We are going to run several of them in parallel.
pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

