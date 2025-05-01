use std::error::Error;
use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ColumnVal {
    One(String),
    Two(bool),
    Three(f64),
    Four(i64),
}

impl ColumnVal::One(val) for ColumnVal {
    fn unwrap(&self) -> String {
        return val as String
    }
}

impl ColumnVal::Two(val) for ColumnVal {
    fn unwrap(&self) -> bool {
        return val as bool
    }
}

impl ColumnVal::Three(val) for ColumnVal {
    fn unwrap(&self) -> f64 {
        return val as f64
    }
}

impl Four(val) for ColumnVal {
    fn unwrap(&self) -> i64 {
        return val as i64
    }
}

pub type Label = String;

#[derive(Debug, Clone)]
pub struct DataFrame {
    data: HashMap<Label, Vec<ColumnVal>>,
    labels: Vec<Label>,
}

#[derive(Debug)]
struct MyError(String);
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}
impl Error for MyError {}

impl DataFrame {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            labels: Vec::new(),
        }
    }

    fn read_csv(&mut self, path: &str, types: &Vec<u32>) -> Result<(), Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b',')
            .has_headers(false)
            .flexible(true)
            .from_path(path)?;
        let mut first_row = true;
        for result in rdr.records() {
            let r = result.unwrap();
            let mut row: Vec<ColumnVal> = vec![];
            if first_row {
                for elem in r.iter() {
                    self.labels.push(elem.to_string());
                }
                first_row = false;
                continue;
            }
            for (i, elem) in r.iter().enumerate() {
                match types[i] {
                    1 => row.push(ColumnVal::One(elem.to_string())),
                    2 => row.push(ColumnVal::Two(elem.parse::<bool>().unwrap())),
                    3 => row.push(ColumnVal::Three(elem.parse::<f64>().unwrap())),
                    4 => row.push(ColumnVal::Four(elem.parse::<i64>().unwrap())),
                    _ => return Err(Box::new(MyError("Unknown type".to_string()))),
                }                        
            }
            for (j, item) in row.iter().enumerate() {
                let whichcol: &String = &self.labels[j];
                match self.data.get_mut(whichcol) {
                    Some(col) => col.push(item.clone()),
                    None => {self.data.insert(whichcol.clone(), vec![item.clone()]);}
                }
            }
        }
        Ok(())
    }


    fn print(&self) {
        for label in &self.labels {
            print!("{:<12}", label)
        } println!("");
        if let Some(name_vec) = &self.data.get("Name") {
            for i in 0..name_vec.len() {
                for j in &self.labels {
                    match &self.data[j][i] {
                        ColumnVal::One(x) => print!("{:<12}", x),
                        ColumnVal::Two(x) => print!("{:<12}", x),
                        ColumnVal::Three(x) => print!("{:<12}", x),
                        ColumnVal::Four(x) => print!("{:<12}", x),
                    }
                } println!("");
            }
        }
    }

    
    fn add_column(&self, new_data:Vec<&str>, label:&str, col_val_number: u32) -> Result<DataFrame, Box<dyn Error>> {
        let mut output = self.clone();
        if &new_data.len() == &self.data["Name"].len() {
            output.labels.push(label.to_string());
            let mut vec = Vec::new();
            for i in new_data {
                match col_val_number {
                    1 => vec.push(ColumnVal::One(i.to_string())),
                    2 => vec.push(ColumnVal::Two(i.parse::<bool>().unwrap())),
                    3 => vec.push(ColumnVal::Three(i.parse::<f64>().unwrap())),
                    4 => vec.push(ColumnVal::Four(i.parse::<i64>().unwrap())),
                    _ => return Err(Box::new(MyError("Unknown type".to_string()))),
                }                       
            } output.data.insert(label.to_string(), vec);
        } else {
            println!("dataframe has {:?} rows, new column has {:?} rows", &self.data["Name"].len(), &new_data.len());
            return Err(Box::new(MyError("Inconsistant number of rows".to_string())));
        } return Ok(output)
    }

    fn merge_frame(&self, other_frame: &DataFrame) -> Result<DataFrame, Box<dyn Error>> {
        let mut output = self.clone();
        if (&other_frame.labels, &other_frame.data.len()) == (&self.labels, &self.data.len()){
            for (key, value) in &other_frame.data {
                if let Some(x) = output.data.get_mut(key) {
                    match (&x[0], &value[0]) {
                        (&ColumnVal::One(_), &ColumnVal::One(_)) => {},
                        (&ColumnVal::Two(_), &ColumnVal::Two(_)) => {},
                        (&ColumnVal::Three(_), &ColumnVal::Three(_)) => {},
                        (&ColumnVal::Four(_), &ColumnVal::Four(_)) => {},
                        _ => return Err(Box::new(MyError("mismatching types".to_string()))),
                        } for y in value {
                            x.push(y.clone());
                        }
                    } 
                }
        } else {return Err(Box::new(MyError("the columns are not consistant for both dataframes".to_string())))}
        
        return Ok(output)
    }

    fn find_column(&self, label: &str) -> Result<(Label, Vec<ColumnVal>), Box<dyn Error>> {
        let out1: Label = label.to_string().clone();
        match &self.data.get(label) {
            Some(out2) => return Ok((out1, out2.to_vec())),
            None => return Err(Box::new(MyError("column does not exist".to_string()))),
        }
    }

    fn restrict_columns(&self, cols: Vec<&str>) -> Result<DataFrame, Box<dyn Error>> {
        let mut new_dataframe: DataFrame = DataFrame::new();
        for col_name in cols {
            match &self.find_column(col_name) {
                Ok((label, new_data)) => new_dataframe.data.insert(label.clone(), new_data.clone()),
                _ => return Err(Box::new(MyError("invalid column".to_string()))),
            };
            new_dataframe.labels.push(col_name.to_string().clone());
        } return Ok(new_dataframe);
    }

    fn filter(
        &self,
        label: &str,
        operation: fn(&ColumnVal) -> bool,
    ) -> Result<Self, Box<dyn Error>> {
        let mut new_dataframe = DataFrame {
            labels: self.labels.clone(),
            data: self.labels.iter().map(|l| (l.clone(), Vec::new())).collect(),
        };
        if let Some(col) = self.data.get(&label.to_string()) {
            for (i, elem) in col.iter().enumerate() {
                if operation(elem) {
                    for c in &self.labels {
                        if let Some(value) = new_dataframe.data.get_mut(c) {
                            if let Some(data) = self.data.get(c) {
                                let this_data = data[i].clone();
                                value.push(this_data);
                            }
                            
                        }
                    }
                    
                }
            }
        } else {
            return Err(Box::new(MyError("not a valid column".to_string())))
        } return Ok(new_dataframe)
         
    }

    fn column_op(
        &self,
        labels: &[String],
        op: fn(&[Vec<ColumnVal>]) -> Vec<ColumnVal>,
    ) -> Vec<ColumnVal> {
        let mut columns = Vec::new();
        for label in labels {
            if let Ok((_col, col_vec)) = self.find_column(&label) {
                columns.push(col_vec);
            }; 
        }let output: Vec<ColumnVal> = op(&columns);
        return output
    }
}
