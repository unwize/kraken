mod data;
mod errors;

use std::env;
use std::path::Path;
use tokio_stream::StreamExt;
use tokio::fs::File;
use crate::errors::KrakenError::IoError;

#[tokio::main]
async fn main() -> miette::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.is_empty() {
        println!("Invalid arguments: Must supply path to data csv")
    }

    let path = Path::new(&args[0]);
    if !path.exists() {
        return Err(IoError)?
    }



    Ok(())
}
