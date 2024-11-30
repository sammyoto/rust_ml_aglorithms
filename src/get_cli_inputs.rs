use std::io::stdin;
use std::path::Path;

pub fn get_cli_inputs(debug: bool) -> (String, String) {
    // Set variables
    let mut model: String = String::new();
    let mut data_location: String = String::new();

    // When debug don't take command line inputs
    if debug {
        model = "lr".to_string();
        data_location = "src/test_data/testing.csv".to_string();
    } else {
        // Get model type
        println!("Enter model type: (lr, mlp (unsupported))");
        stdin().read_line(&mut model).unwrap();
        // Get rid of newline
        model.pop();

        // Get data file
        println!("Enter location of data file:");
        stdin().read_line(&mut data_location).unwrap();
        // Get rid of newline
        data_location.pop();
    }

    // validate inputs
    let input_vals: (bool, bool) = validate_inputs((&model, &data_location));

    if !input_vals.0 {
        panic!("Please input lr or mlp!");
    }
    if !input_vals.1 {
        panic!("Invalid file location!");
    }

    (model, data_location)
}

fn validate_inputs(inputs: (&String, &String)) -> (bool, bool){
    let mut input_val: (bool, bool) = (true, true); 
    // Test model type
    if inputs.0 != "lr" && inputs.0 != "mlp" {
        input_val.0 = false;
    }
    // Test data file
    if !Path::new(&inputs.1).exists() {
        input_val.1 = false;
    }
    
    input_val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_true() {
        let model: String = String::from("lr");
        let path: String = String::from("src/test_data/test.csv");

        let inputs: (&String, &String) = (&model, &path);
        let result: (bool, bool) = validate_inputs(inputs);
        assert_eq!(result, (true, true));
    }

    #[test]
    fn both_false() {
        let model: String = String::from("frankModel");
        let path: String = String::from("does_not_exist.txt");

        let inputs: (&String, &String) = (&model, &path);
        let result: (bool, bool) = validate_inputs(inputs);
        assert_eq!(result, (false, false));
    }
}