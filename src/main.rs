fn main() {
    let returned_data = some_function(12.02, 55);

    some_procedure(12.5, 32);

    let string_slice_var: &str = "ola";
    some_str_procedure(string_slice_var);

    let string_var = String::from("terra system real string");
    some_str_procedure(&string_var);

    some_string_procedure(string_var);

    println!("{}", returned_data);
}


fn some_function(params_a: f32, params_b: i128) -> f32 {
    // params_a + params_b
    println!("hello functions");

    let return_value = 10.1 * params_a + params_b as f32;// cast

    return_value
}

fn some_procedure(params_a: f32, params_b: i128) {
    println!("hello world: {}:{}", params_a, params_b);
}

fn some_str_procedure(params: &str) {
    println!("a string slice {}", params);
}

fn some_string_procedure(params: String) {
    println!("a string {}", params);
}