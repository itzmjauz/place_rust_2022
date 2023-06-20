use csv::ReaderBuilder;
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub timestamp: String,
    _user_id: String,
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
