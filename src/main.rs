extern crate actix;
extern crate actix_web;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
#[macro_use] extern crate log;
extern crate r2d2;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::env;

use actix::prelude::*;
use actix_web::{
    http, middleware, server, App, AsyncResponder, FutureResponse, HttpRequest, HttpResponse,
};

use diesel::prelude::*;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use futures::Future;

mod db;
mod models;
mod schema;

use db::{
    DbExecutor,
    FindThing,
};


// This struct represents state. In this example each thread (4 threads by default on my laptop)
// gets its own state.
struct AppState {
    db: Addr<DbExecutor>,
}


fn index(req: &HttpRequest<AppState>) -> String {
    let resp = format!("Request RemoteIP={:?}", req.connection_info().remote());
    info!("{}", resp);
    resp
}

// Async handler
fn thing(req: &HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let name: String  = req.match_info().query("name").unwrap();

    info!("Request for thing called '{}'", name);

    // send async `FindThing` message to a `DbExecutor`
    req.state()
        .db
        .send(FindThing {
            name: name,
        })
        .from_err()
        .and_then(|res| match res {
            Ok(Some(thing)) => Ok(HttpResponse::Ok().json(thing)),
            Ok(None) => Ok(HttpResponse::NotFound().into()),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}


fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let listen = env::var("LISTEN_ADDR").unwrap();
    let dburl = env::var("DATABASE_URL").unwrap();
    let sys = actix::System::new("hello-rust");

    // Start 3 db executor actors
    let manager = ConnectionManager::<PgConnection>::new(dburl);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let addr = SyncArbiter::start(3, move || DbExecutor(pool.clone()));

    server::new(move || {
        App::with_state(AppState { db: addr.clone() })
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.method(http::Method::GET).f(index))
            .resource("/thing/{name}", |r| r.method(http::Method::GET).f(thing))
    }).bind(&listen)
        .unwrap()
        .start();
    info!("Started http server: http://{}", listen);

    let _ = sys.run();
}

