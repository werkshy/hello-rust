use actix::prelude::*;
use actix::dev::{MessageResponse, ResponseChannel};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub mod thing;

// This is db executor actor. We are going to run several of them in parallel.
pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub trait TracingHandler<M> : Handler<M>
where
    Self: Actor,
    M: Message,
{
    type WrappedResult: MessageResponse<Self, M>;
    fn inner_handle(&mut self, msg: M, ctx: &mut Self::Context) -> Self::WrappedResult;
    //fn inner_handle(&mut self, msg: M, ctx: &mut Self::Context) -> Self::Result;
}

// Wrapped response type
pub struct TracedResult<T> {
    result: T,
    span: i32
}



// Generic Handler implementation that calls the inner handler defined for each message type, and
// performs tracing.
impl <M> Handler<M> for DbExecutor
where
    DbExecutor: TracingHandler<M>,
    M: Message,
{
    // TODO: return a result that includes the tracing
    //type Result = TracedResult<<DbExecutor as TracingHandler<M>>::WrappedResult>;
    //type Result = <DbExecutor as TracingHandler<M>>::WrappedResult;
    type Result = TracedResult<<DbExecutor as Handler<M>>::Result>;

    fn handle(&mut self, msg: M, ctx: &mut Self::Context) -> Self::Result {
        info!("Executing inner handle");
        let result = DbExecutor::inner_handle(self, msg, ctx);
        
        let returnval = TracedResult { result: result, span: 1 };

        returnval
        
        //result
    }
}

/*

// Attempt to implement MessageResponse by calling into the wrapped result type
//impl<A, M, T, I: 'static, E: 'static> MessageResponse<A, M> for TracedResult<T>
impl<A, M, T, I: 'static, E: 'static> MessageResponse<A, M> for TracedResult<T>
where
  A: Actor,
  M: Message<Result = Result<I, E>>,
  T: MessageResponse<A, M>,
{
    fn handle<R: ResponseChannel<M>>(self, ctx: &mut A::Context, tx: Option<R>) {
        self.result.handle(ctx, tx)
    }
}
*/


#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    // Used in other tests in this module
    pub fn get_conn() -> PgConnection {
        dotenv::dotenv().ok();
        let dburl = env::var("DATABASE_URL").unwrap();
        diesel::pg::PgConnection::establish(&dburl).unwrap()
    }
}
