mod data;
mod errors;

use std::collections::HashMap;
use std::env;
use std::path::Path;

use crate::data::Transaction;
use crate::errors::KrakenError;
use polars::prelude::*;
use anyhow::Result;
use crate::errors::KrakenError::Error;

// I debated between this LazyFrame implementation and streaming with `csv-async`. This was far less
// verbose and might actually tolerate very-large datasets.
// Docs: https://docs.pola.rs/user-guide/io/csv/#read-write
fn parse_csv(file_in: &str) -> Result<LazyFrame> {
    Ok(LazyCsvReader::new(PlPath::new(file_in))
        .finish()?)
}


fn main()  -> Result<()>{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Invalid arguments: Must supply path to data csv");
        Err(Error)?
    }

    let path = Path::new(&args[1]);
    if !path.exists() {
        Err(KrakenError::IO)?
    }

    let lazy_data: LazyFrame = parse_csv(path.to_str().unwrap())?;
    let client_accounts: HashMap<u16, HashMap<u32, Vec<Box<dyn Transaction>>>> = HashMap::new();
    println!("{}", lazy_data.with_row_index("index", None).group_by([col("client")]).agg([col("type"), col("tx"), col("amount"), col("index")]).collect()?);

    Ok(())
}
