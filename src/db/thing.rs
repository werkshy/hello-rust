use actix::prelude::*;
use actix_web::*;
use diesel::prelude::*;

use models::thing::Thing;
use schema;

use db::DbExecutor;
use db::TracingHandler;

// Message Definitions
pub struct FindThing {
    pub name: String,
}

impl Message for FindThing {
    type Result = Result<Option<Thing>, Error>;
}

impl TracingHandler<FindThing> for DbExecutor {
    type WrappedResult = Result<Option<Thing>, Error>;

    fn inner_handle(&mut self, msg: FindThing, _: &mut Self::Context) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();
        find_thing(conn, msg)
    }
}

fn find_thing(conn: &PgConnection, msg: FindThing) -> Result<Option<Thing>, Error> {
    use self::schema::things::dsl::*;
    let mut items = things
        .filter(name.eq(&msg.name))
        .limit(1)
        .load::<Thing>(conn)
        .map_err(|_| error::ErrorInternalServerError("Error loading thing"))?;
    Ok(items.pop())
}

#[cfg(test)]
mod tests {
    use super::*;
    use db::tests::get_conn;

    #[test]
    fn get_thing_when_it_exists() {
        let conn = get_conn();

        let msg = FindThing { name: "foo".to_string() };
        let result = find_thing(&conn, msg);
        assert!(result.is_ok(), "DB error");

        let result_opt = result.unwrap();
        assert!(result_opt.is_some());
        assert_eq!(result_opt.unwrap().name, "foo");
    }
}
