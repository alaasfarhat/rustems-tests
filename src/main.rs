use polars_core::prelude::*;
use polars_io::prelude::*;

fn example(path_to_file: &str) -> PolarsResult<DataFrame> {
    CsvReader::from_path(path_to_file)?
            .has_header(true)
            .finish()
}

fn main() {
    let spots_path = "/home/alaa/repos/rustems-tests/data/spots.csv";
    let solar_path = "/home/alaa/repos/rustems-tests/data/solar_data.csv";
    let cons_path = "/home/alaa/repos/rustems-tests/data/consumption_data.csv";
    
    let spot_df = example(spots_path);
    let solar_df = example(solar_path);
    let cons_df = example(cons_path);


    println!("{:?}", cons_df);
}