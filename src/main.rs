extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
#[macro_use]
extern crate log;
extern crate r2d2;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::env;

use actix::prelude::*;
use actix_web::{
    fs, http, middleware, server, App, AsyncResponder, FutureResponse, HttpRequest, HttpResponse,
};

use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use futures::Future;

mod db;
mod models;
mod schema;

use db::{thing::FindThing, DbExecutor};

// This struct represents state. In this example each thread (4 threads by default on my laptop)
// gets its own state.
struct AppState {
    db: Addr<DbExecutor>,
}

fn index<AppState: 'static>(_req: &HttpRequest<AppState>) -> fs::NamedFile {
    fs::NamedFile::open("./static/index.html").unwrap()
}

// Async handler
fn thing(req: &HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let name: String = req.match_info().query("name").unwrap();

    // send async `FindThing` message to a `DbExecutor`
    req.state()
        .db
        .send(FindThing { name: name })
        .from_err()
        .and_then(|res| match res {
            Ok(Some(thing)) => Ok(HttpResponse::Ok().json(thing)),
            Ok(None) => Ok(HttpResponse::NotFound().into()),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

// Start DB executors
fn db_executors() -> Addr<DbExecutor> {
    let num_executors: usize = env::var("DATABASE_EXECUTORS").unwrap().parse().unwrap();
    let dburl = env::var("DATABASE_URL").unwrap();
    let manager = ConnectionManager::<PgConnection>::new(dburl);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    SyncArbiter::start(num_executors, move || DbExecutor(pool.clone()))
}

fn routes(app: App<AppState>) -> App<AppState> {
    app.default_resource(|_| HttpResponse::NotFound())
        .handler("/static", fs::StaticFiles::new("./static").unwrap())
        .resource("/", |r| r.method(http::Method::GET).f(index))
        .resource("/thing/{name}", |r| r.method(http::Method::GET).f(thing))
}

fn middleware(app: App<AppState>) -> App<AppState> {
    let log_format = "%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %Dms";
    app.middleware(middleware::Logger::new(log_format))
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let sys = actix::System::new("hello-rust");

    let db_addr = db_executors();

    let listen = env::var("LISTEN_ADDR").unwrap();

    server::new(move || {
        App::with_state(AppState {
            db: db_addr.clone(),
        })
        .configure(routes)
        .configure(middleware)
    })
    .bind(&listen)
    .expect("Can not bind to listen address")
    .start();
    info!("Started http server: http://{}", listen);

    let _ = sys.run();
}
