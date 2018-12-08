use ::actix::dev::MessageResponse;
use ::actix::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use log::info;

pub mod thing;

// This is db executor actor. We are going to run several of them in parallel.
pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub trait TracingHandler<M>: Handler<M>
where
    Self: Actor,
    M: Message,
{
    type WrappedResult: MessageResponse<Self, M>;
    fn inner_handle(&mut self, msg: M, ctx: &mut Self::Context) -> Self::Result;
}

pub trait TracingMessage: Message {}

impl<M> Handler<M> for DbExecutor
where
    DbExecutor: TracingHandler<M>,
    M: Message,
{
    type Result = <DbExecutor as TracingHandler<M>>::WrappedResult;

    fn handle(&mut self, msg: M, ctx: &mut Self::Context) -> Self::Result {
        info!("Executing inner handle");
        DbExecutor::inner_handle(self, msg, ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    // Used in other tests in this module
    pub fn get_conn() -> PgConnection {
        dotenv::dotenv().ok();
        let dburl = env::var("DATABASE_URL").unwrap();
        diesel::pg::PgConnection::establish(&dburl).unwrap()
    }
}
