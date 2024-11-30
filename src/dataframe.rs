use std::fs::File;
use std::io::{self, BufRead};

pub struct DataFrame {
    pub columns: Vec<String>,
    pub data: Vec<Vec<f32>>,
    pub target: Vec<f32>
}

impl DataFrame {
    pub fn new() -> Self {
        Self {columns: Vec::new(), data: Vec::new(), target: Vec::new()}
    }

    pub fn load_data(&mut self, data_location: String) {
        // Data vectors
        let mut columns: Vec<String> = Vec::new();
        let mut data: Vec<Vec<f32>> = Vec::new();

        // Load data from file
        let file: Result<File, io::Error> = File::open(data_location);
        let reader: io::BufReader<File> = io::BufReader::new(file.unwrap());

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
                for i in 0..vals.len() {
                    let val_num: f32 = vals[i].to_string().parse().unwrap();
                    // last value of row is the target
                    if i == vals.len() - 1 {
                        self.target.push(val_num);
                    } else {
                        row.push(val_num);
                    }
                }
                data.push(row);
            }
        }

        // Set self to retrieved data
        self.columns = columns;
        self.data = data;
    }

    pub fn standardize_data(&mut self) {
        // check if data length is long enough
        if self.data.len() < 2 {
            return
        }
        // col
        for i in 0..self.data[0].len() {

            let col: Vec<f32> = self.get_column(i);

            // get mean and std dev of column
            let col_sum: f32 = col.iter().sum();
            let mean: f32 = col_sum/col.len() as f32;
            
            //std dev =  sqrt(sum(col - mean)^2)
            let mut std_dev: f32 = 0.0;
            for k in 0..col.len() {
                std_dev += (col[k] - mean).powi(2);
            }
            std_dev = (std_dev/col.len() as f32).sqrt();
            // row
            for j in 0..self.data.len(){
                self.data[j][i] = (self.data[j][i] - mean) / std_dev;
            }
        }

        // do target as well
        let col: &Vec<f32> = &self.target;
        // get mean and std dev of column
        let col_sum: f32 = col.iter().sum();
        let mean: f32 = col_sum/col.len() as f32;
        
        //std dev =  sqrt(sum(col - mean)^2)
        let mut std_dev: f32 = 0.0;
        for k in 0..col.len() {
            std_dev += (col[k] - mean).powi(2);
        }
        std_dev = (std_dev/col.len() as f32).sqrt();
        // target
        for j in 0..self.target.len() {
            self.target[j] = (self.target[j] - mean) / std_dev;
        }
    }

    pub fn get_row(&self, row: usize) -> &Vec<f32> {
        &self.data[row]
    }

    // data isn't stored in columns, will return a new vector not a reference to data
    pub fn get_column(&self, column: usize) -> Vec<f32> {
        let mut col: Vec<f32> = Vec::new();

        for i in 0..self.data.len() {
            col.push(self.data[i][column]);
        }

        col
    }
    
    pub fn print(&self) {
        // Print column headers
        println!("Data:");
        print!("{:<15}", "row_num");
        for col in &self.columns {
            print!("{:<15}", col);
        }
        println!("");

        // Print rows
        let mut row_num = 0;
        for row in &self.data {
            print!("{:<15}", row_num);
            for data in row {
                print!("{:<15}", data);
            }
            // print target column
            print!("{:<15}", self.target[row_num]);
            row_num += 1;
            print!("\n");
        }
        println!("\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_data() {
        // setup dataframe with test data
        let mut test_dataframe: DataFrame = DataFrame::new();
        test_dataframe.load_data("src/test_data/test.csv".to_string());
        // setup mock data and target
        let mut test_data: Vec<Vec<f32>> = Vec::new();
        test_data.push(vec![2.0, 3.0]);
        let mut test_target: Vec<f32> = Vec::new();
        test_target.push(4.0);

        assert_eq!(&test_dataframe.data, &test_data);
        assert_eq!(&test_dataframe.target, &test_target);
    }

    #[test]
    fn test_get_row_0() {
        // setup dataframe with test data
        let mut test_dataframe: DataFrame = DataFrame::new();
        test_dataframe.load_data("src/test_data/test.csv".to_string());
        // get row 0
        let row_0: &Vec<f32> = test_dataframe.get_row(0);
        // create empty test vec and add numbers
        let mut test_vec: Vec<f32> = Vec::new();
        test_vec.push(2.0);
        test_vec.push(3.0);
        assert_eq!(row_0, &test_vec);
    }

    #[test]
    fn test_get_column_0() {
        // setup dataframe with test data
        let mut test_dataframe: DataFrame = DataFrame::new();
        test_dataframe.load_data("src/test_data/test.csv".to_string());
        // get col 0
        let col_0: Vec<f32> = test_dataframe.get_column(0);
        // create empty test vec and add numbers
        let mut test_vec: Vec<f32> = Vec::new();
        test_vec.push(2.0);
        assert_eq!(col_0, test_vec);
    }
}