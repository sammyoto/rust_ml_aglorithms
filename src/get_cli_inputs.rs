use std::io::stdin;

pub fn get_cli_inputs() -> (String, String) {
    // Get model type
    let mut model = String::new();
    println!("Enter model type: (lr, mlp)");
    stdin().read_line(&mut model).unwrap();
    // Get rid of newline
    model.pop();

    // Get data file
    let mut data_location = String::new();
    println!("Enter location of data file:");
    stdin().read_line(&mut data_location).unwrap();
    // Get rid of newline
    data_location.pop();

    (model, data_location)
}