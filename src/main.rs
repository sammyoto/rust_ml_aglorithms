use dataframe::DataFrame;

mod get_cli_inputs;
mod dataframe;

fn main() {
    // get inputs
    let inputs: (String, String) = get_cli_inputs::get_cli_inputs(true);
    println!("\nModel Type: {}, Data Location: {}\n", inputs.0, inputs.1);

    // load data
    let mut data: DataFrame = DataFrame::new();
    data.load_data(inputs.1);
    data.print();
}
