use std::fs::File;
use std::io::{self, BufRead};

pub struct DataFrame {
    columns: Vec<String>,
    data: Vec<Vec<f32>>
}

impl DataFrame {
    pub fn new() -> Self {
        Self {columns: Vec::new(), data: Vec::new()}
    }

    pub fn load_data(&mut self, data_location: String) {
        // Data vectors
        let mut columns: Vec<String> = Vec::new();
        let mut data: Vec<Vec<f32>> = Vec::new();

        // Load data from file
        let file: Result<File, io::Error> = File::open(data_location);
        let reader = io::BufReader::new(file.unwrap());

        // Write data into vectors
        let mut first_line: bool = true;
        for (_index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let vals: Vec<&str> = line.split(",").collect();
            if first_line {
                for val in vals {
                    columns.push(val.to_string());
                }
                first_line = false;
            } else {
                let mut row: Vec<f32> = Vec::new();
                for val in vals {
                    let val_num: f32 = val.to_string().parse().unwrap();
                    row.push(val_num);
                }
                data.push(row);
            }
        }

        // Set self to retrieved data
        self.columns = columns;
        self.data = data;
    }
    
    pub fn print(&self) {
        // Print column headers
        println!("Data:");
        print!("{:<10}", "row_num");
        for col in &self.columns {
            print!("{:<10}", col);
        }
        println!("");

        // Print rows
        let mut row_num = 0;
        for row in &self.data {
            print!("{:<10}", row_num);
            for data in row {
                print!("{:<10}", data);
            }
            row_num += 1;
            print!("\n");
        }
        println!("\n");
    }
}