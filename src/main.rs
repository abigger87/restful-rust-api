// ** src/main.rs

#[macro_use]
extern crate diesel;

// ** local modules
mod logger;
mod models;
mod schema;

type StdErr = Box<dyn std::error::Error>;

fn main() -> Result<(), StdErr> {
    // ** Load Environment Variables
    dotenv::dotenv()?;

    // ** Initialize logger
    logger::init()?;

    // ** Test variables
    assert_eq!("INFO", std::env::var("LOG_LEVEL").unwrap());

    Ok(())
}
