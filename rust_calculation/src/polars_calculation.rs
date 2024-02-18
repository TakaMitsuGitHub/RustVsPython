use polars::prelude::*;
use polars::prelude::CsvReader;
use std::error::Error;


pub fn read_csv() -> Result<(), Box<dyn Error>> {
    let df = CsvReader::from_path("../test.csv")?
        .infer_schema(None)
        .finish()?;

    // Print the DataFrame
    println!("{:?}", df);

    Ok(())
}