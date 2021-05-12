// ** src/main.rs

type StdErr = Box<dyn std::error::Error>;

fn main() -> Result<(), StdErr> {
    // loads env variables from .env
    dotenv::dotenv()?;

    // example
    assert_eq!("INFO", std::env::var("LOG_LEVEL").unwrap());

    Ok(())
}
