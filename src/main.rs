fn main() {
    // string slice
    let example_str: &str = "terra";

    let example_string: String = String::from("terrasystem");

    println!("{}", example_str);
    println!("{}", example_string);

    //translate between string and string slice
    let string_from_str: String = example_str.to_string();
    let string_from_str2: String = "some hardecoded".to_string();

    println!("{}", string_from_str);
    println!("{}", string_from_str2);

    let string_from_hardecoded = String::from("some hardcoded");
    let string_from_str_var = String::from(example_str);

    println!("{}", string_from_hardecoded);
    println!("{}", string_from_str_var);

    let str_from_string: &str = &example_string;

    println!("{}", str_from_string);

    //concatenate 2 string
    let combine_string_literals = ["first", "second"].concat();
    let combine_with_format_macro = format!("{} {}", "first", "second");

    println!("{}", combine_string_literals);
    println!("{}", combine_with_format_macro);

    let string_plus_str = example_string + example_str;

    println!("{}", string_plus_str);

    let mut mut_string = String::new();
    mut_string.push_str(example_str);
    mut_string.push_str("example_str");
    mut_string.push('c');

    let a = String::from("a");
    let b = String::from("b");
    let combine = a + &b + &mut_string;

    println!("{}",combine);

    // let char_by_index = &example_str.chars().nth(1);

    // println!("{}", char_by_index); //errro

}
