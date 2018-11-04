use actix::prelude::*;
use actix_web::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use models::thing::Thing;
use schema;

use db::DbExecutor;

// Message Definitions
pub struct FindThing {
    pub name: String,
}

impl Message for FindThing {
    type Result = Result<Option<Thing>, Error>;
}

impl Handler<FindThing> for DbExecutor {
    type Result = Result<Option<Thing>, Error>;

    fn handle(&mut self, msg: FindThing, _: &mut Self::Context) -> Self::Result {
        use self::schema::things::dsl::*;

        let conn: &PgConnection = &self.0.get().unwrap();

        let mut items = things
            .filter(name.eq(&msg.name))
            .limit(1)
            .load::<Thing>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading thing"))?;
        Ok(items.pop())
    }
}
