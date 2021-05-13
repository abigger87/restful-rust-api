# restful-rust-api

Restful API built with Rust from [pretzelhammer](https://github.com/pretzelhammer) rust blog at [restful-api-in-sync-and-async-rust](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md)

## Migrations

Prerequisite: install diesel cli with `cargo install diesel_cli --no-default-features --features "postgres sqlite mysql"`

Run diesel migrations with `diesel migration run`

NOTE: You must have a database running at the connection string provided in the `.env` variable `DATABASE_URL`
