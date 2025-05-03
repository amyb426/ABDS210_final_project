//! will read the csv, take the data specified, and output a vector of rows

use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use serde::Deserialize;

//this struct is the form in which the csv is deserialized into, with option enum. This struct is private
//The FullRecord struct is used in:
    //for result in rdr.deserialize() {
        //let record: FullRecord = result?;
#[derive(Debug, Deserialize)]
struct FullRecord {
    #[serde(rename = "Museum Type")]
    museum_type: Option<String>,
    #[serde(rename = "Locale Code (NCES)")]
    locale: Option<i8>,
    #[serde(rename = "State Code (FIPS)")]
    state_code: Option<i8>,
    #[serde(rename = "Region Code (AAM)")]
    region_code: Option<i8>,
    #[serde(rename = "Revenue")]
    revenue: Option<f64>,
}

//this struct, Row, is used in the main file, so it is public.
//The Row struct contains the data I am interested in for one museum
//all but the revenue is in numberical categories that correspond to different types, areas, etc.
#[derive(Debug, Clone)]
pub struct Row {
    pub museum_type: i8,
    pub locale: i8,
    pub state_code: i8,
    pub region_code: i8,
    pub revenue: f64,
}

//there are two functions in the module new() (method of Row) and load(). Both these methods are used together to make instances of Row from the csv file
impl Row {
    //the new method takes data, makes the museum type into a numeric value (using match), and creates an instance of Row with the data as its fields
    pub fn new(a: String, b: i8, c: i8, d: i8, e: f64) -> Self {
        let type_code = match a.as_str() {
            "HISTORIC PRESERVATION" => 1,
            "GENERAL MUSEUM" => 2,
            "ART MUSEUM" => 3,
            "HISTORY MUSEUM" => 4,
            "ARBORETUM, BOTANICAL GARDEN, OR NATURE CENTER" => 5,
            _ => 0,
        };
        Self {
            museum_type: type_code,
            locale: b,
            state_code: c,
            region_code: d,
            revenue: e,
        }
    }
}

//the load function is used to read a csv file, convert it into a vector of Rows
//input: name of the csv file
//output: either Ok(vector of Row instances) or Err
pub fn load(filename: &str) -> Result<Vec<Row>, Err> {
    //uses std::File crate to open and read csv, csv and serde have been added to Cargo.toml to read, deserialize the csv
    let file = File::open(filename)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    //this will be output if all goes well
    let mut filtered_data = Vec::new();

    for result in rdr.deserialize() {
        let record: FullRecord = result?;

        //only make into a Row if there is data for the following (lazy dataset analysis)
        if let Some(revenue) = record.revenue {
            if record.museum_type.is_some() &&
               record.locale.is_some() &&
               record.state_code.is_some() &&
               record.region_code.is_some() {
                //making instance of Row
                let row = Row::new(
                    record.museum_type.unwrap(),
                    record.locale.unwrap(),
                    record.state_code.unwrap(),
                    record.region_code.unwrap(),
                    revenue,
                ); //add the Row instance to the filtered_data vector
                filtered_data.push(row);
            }
        }
    } println!("{:?} converted to {:?} rows", filename, filtered_data.len());
    Ok(filtered_data)
}
