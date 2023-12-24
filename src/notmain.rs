use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use polars::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    // Specify the path to your CSV files
    let file_path = "/home/alaa/repos/rustems-tests/data/spots.csv";
    let cons_path = "/home/alaa/repos/rustems-tests/data/consumption_data.csv";
    let prod_path = "/home/alaa/repos/rustems-tests/data/solar_data.csv";

    // Open the CSV files
    let file = File::open(file_path)?;
    let file_1 = File::open(cons_path)?;
    let file_2 = File::open(prod_path)?;

    // Create CSV readers using the ReaderBuilder
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut csv_reader_cons = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file_1);

    let mut csv_reader_prod = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file_2);

    // Process the first CSV file (spots.csv)
    let spots_records: Vec<csv::StringRecord> = csv_reader.records().collect::<Result<_, _>>()?;
    let spots_data: Vec<Vec<&str>> = spots_records.iter().map(|record| record.iter().collect()).collect();
    let spots_df = DataFrame::new(spots_data)?;

    // Process the second CSV file (consumption_data.csv)
    let cons_records: Vec<csv::StringRecord> = csv_reader_cons.records().collect::<Result<_, _>>()?;
    let cons_data: Vec<Vec<&str>> = cons_records.iter().map(|record| record.iter().collect()).collect();
    let cons_df = DataFrame::new(cons_data)?;

    // Process the third CSV file (solar_data.csv)
    let prod_records: Vec<csv::StringRecord> = csv_reader_prod.records().collect::<Result<_, _>>()?;
    let prod_data: Vec<Vec<&str>> = prod_records.iter().map(|record| record.iter().collect()).collect();
    let prod_df = DataFrame::new(prod_data)?;

    // Display the DataFrames
    println!("Spots DataFrame:\n{:?}", spots_df);
    println!("Consumption DataFrame:\n{:?}", cons_df);
    println!("Solar DataFrame:\n{:?}", prod_df);

    Ok(())
}