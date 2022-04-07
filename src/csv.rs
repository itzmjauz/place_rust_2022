use csv::ReaderBuilder;
use serde::Deserialize;
use std::cmp::max;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub timestamp: String,
    user_id: String,
    pub pixel_color: String,
    pub coordinate: String,
}

pub fn csv_iter(path: &str) -> Box<dyn Iterator<Item = Record>> {
    let file = File::open(path).expect("Failed to open file");
    let rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file)
        .into_deserialize::<Record>()
        .map(|row| row.expect("Failed to parse row"));

    Box::new(rdr)
}

#[allow(dead_code)]
pub fn print_lines(path: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;

    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }

    Ok(())
}
