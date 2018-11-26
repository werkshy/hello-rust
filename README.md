Hello Rust
==========

This is a simple actix-web rust API example using Diesel as the ORM. It's mostly copy-pasted from https://github.com/actix/examples/tree/master/diesel for learning purposes.


## To get started

Check out the config file `.env` which will look something like this:

```
RUST_LOG=hello_rust=debug,actix_web::middleware::logger=info
LISTEN_ADDR=127.0.0.1:8088
DATABASE_URL=postgres://localhost/hello_rust
DATABASE_EXECUTORS=4
```

Initialize the diesel migration system:

```
createdb hello_rust
cargo install diesel_cli --no-default-features --features postgres
diesel setup
```

## To create a new table
```
cargo migration generate create_mytable
# Edit generated files
diesel migration run
```

## Resolve dependencies, build & run
```
cargo run
```

## Use
```
curl http://127.0.0.1:8088/
curl http://127.0.0.1:8088/thing/foo
```

## TODO

- [x] Json http endpoint
- [x] Static files & homepage
- [x] Async ORM (wrapper)
- [ ] Tests
    - [x] db
    - [ ] integration
- [ ] CI
- [ ] Deployment
- [ ] Tracing
- [ ] Structured logging

