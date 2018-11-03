use actix::prelude::*;
use actix_web::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use models;
use schema;

// This is db executor actor. We are going to run several of them in parallel.
// TODO: genericize with connection type
pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

// Message Definitions
pub struct FindThing {
    pub name: String,
}

impl Message for FindThing {
    type Result = Result<models::Thing, Error>;
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<FindThing> for DbExecutor {
    type Result = Result<models::Thing, Error>;

    fn handle(&mut self, msg: FindThing, _: &mut Self::Context) -> Self::Result {
        use self::schema::things::dsl::*;

        let conn: &PgConnection = &self.0.get().unwrap();

        debug!("Looking up Thing called '{:?}'", msg.name);

        let mut items = things
            .filter(name.eq(&msg.name))
            .load::<models::Thing>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading thing"))?;

        // TODO better error handling
        Ok(items.pop().unwrap())
    }
}
