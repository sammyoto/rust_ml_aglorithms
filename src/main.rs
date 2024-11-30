use get_cli_inputs::get_cli_inputs;
use dataframe::DataFrame;
use linear_regressor::LinearRegressor;

mod get_cli_inputs;
mod dataframe;
mod linear_regressor;

fn main() {
    // get inputs
    let inputs: (String, String) = get_cli_inputs(true);
    println!("\nModel Type: {}, Data Location: {}\n", inputs.0, inputs.1);

    // load data
    let mut data: DataFrame = DataFrame::new();
    data.load_data(inputs.1.clone());
    println!("Normal Data");
    data.print();

    // standardize
    // data.standardize_data();
    // println!("Standardized Data");
    // data.print();

    // create model and train
    let mut lr_model = LinearRegressor::new();
    lr_model.fit(data, 100, 0.001);

    // validate model
    let mut test_data: DataFrame = DataFrame::new();
    test_data.load_data(inputs.1.clone());

    for i in 0..5 {
        let prediction: f32 = lr_model.predict(test_data.get_row(i));
       println!("Predicted: {} Actual: {}", prediction, test_data.target[i]);
    }
}
    
