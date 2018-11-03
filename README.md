Hello Rust
==========

This is a simple actix-web rust API example using Diesel as the ORM. It's mostly copy-pasted from https://github.com/actix/examples/tree/master/diesel for learning purposes.


## To get started

```
cargo install diesel_cli
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
curl http://127.0.0.1:8088/thing/
```

## TODO

- tests
