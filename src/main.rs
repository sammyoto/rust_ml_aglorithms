mod get_cli_inputs;

fn main() {
    let inputs: (String, String) = get_cli_inputs::get_cli_inputs();
    println!("Model Type: {}, Data Location: {}", inputs.0, inputs.1);
}
