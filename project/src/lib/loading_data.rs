//! will read the csv, take the data specified, and output a vector of rows

use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FullRecord {
    #[serde(rename = "Museum ID")]
    museum_id: Option<i64>,
    //#[serde(rename = "Museum Name")]
    //museum_name: Option<String>,
    #[serde(rename = "Museum Type")]
    museum_type: Option<String>,
    #[serde(rename = "Locale Code (NCES)")]
    locale: Option<i8>,
    #[serde(rename = "State Code (FIPS)")]
    state_code: Option<i8>,
    #[serde(rename = "Region Code (AAM)")]
    region_code: Option<i8>,
    #[serde(rename = "Revenue")]
    revenue: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct Row {
    pub museum_id: i64,
    //pub museum_name: String,
    pub museum_type: i8,
    pub locale: i8,
    pub state_code: i8,
    pub region_code: i8,
    pub revenue: i64,
}

impl Row {
    pub fn new(id: i64, 
        //name: String, 
        a: String, b: i8, c: i8, d: i8, e: i64) -> Self {
        let type_code = match a.as_str() {
            "HISTORIC PRESERVATION" => 1,
            "GENERAL MUSEUM" => 2,
            "ART MUSEUM" => 3,
            "HISTORY MUSEUM" => 4,
            "ARBORETUM, BOTANICAL GARDEN, OR NATURE CENTER" => 5,
            _ => 0,
        };
        Self {
            museum_id: id,
            //museum_name: name,
            museum_type: type_code,
            locale: b,
            state_code: c,
            region_code: d,
            revenue: e,
        }
    }
}

pub fn load(filename: &str) -> Result<Vec<Row>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut filtered_data = Vec::new();

    for result in rdr.deserialize() {
        let record: FullRecord = result?;

        if let Some(revenue) = record.revenue {
            // Handle missing fields (if necessary, use default values or skip)
            if record.museum_id.is_some() &&
               //record.museum_name.is_some() &&
               record.museum_type.is_some() &&
               record.locale.is_some() &&
               record.state_code.is_some() &&
               record.region_code.is_some() {
                let row = Row::new(
                    record.museum_id.unwrap(),
                    //record.museum_name.unwrap(),
                    record.museum_type.unwrap(),
                    record.locale.unwrap(),
                    record.state_code.unwrap(),
                    record.region_code.unwrap(),
                    revenue,
                );
                filtered_data.push(row);
            }
        }
    } println!("{:?} converted to {:?} rows", filename, filtered_data.len());
    Ok(filtered_data)
}
