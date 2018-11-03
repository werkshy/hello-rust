extern crate actix_web;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate env_logger;

use actix_web::{http, server, App, HttpRequest};
use std::cell::Cell;

// This struct represents state. In this example each thread (4 threads by default on my laptop)
// gets its own state.
struct AppState {
    counter: Cell<usize>,
}


fn index(req: &HttpRequest<AppState>) -> String {
    let count = req.state().counter.get() + 1; // <- get count
    req.state().counter.set(count); // <- store new count in state
    let resp = format!("Request #{} - RemoteIP={:?}", count, req.connection_info().remote());
    info!("{}", resp);
    resp
}


fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let listen = std::env::var("LISTEN_ADDR").unwrap();
    server::new(|| {
        App::with_state(AppState { counter: Cell::new(0) })
        .resource("/", |r| r.method(http::Method::GET).f(index))
    }).bind(&listen)
        .unwrap()
        .run();
}

