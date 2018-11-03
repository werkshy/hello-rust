Hello Rust
==========

This is a simple actix-web rust API example using Diesel as the ORM. It's mostly copy-pasted from https://github.com/actix/examples/tree/master/diesel for learning purposes.


## To get started

```
cargo install diesel_cli
diesel migration run
```

## To create a new table
```
cargo migration generate create_mytable
```
