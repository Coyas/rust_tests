fn main() {
    let some_bool = false;
    let some_int = 30;
    let some_int2 = 50;

    if some_bool == true ||  (some_int > 100 && some_int2 == 200){
        println!("é boolean");
    }else if some_int == 30 {
        println!("é boolean e int 30");
    }else{
        println!("nao é boolean");
    }


    let var_from_inline = if some_int == 30 { 300 } else { 400 };

    println!("{}", var_from_inline);

    //similar to if
    match some_bool{
        true => {
            println!("true");
        }
        false => {
            println!("false");
        }
    }

    match some_int {
        0 => println!("Hit 0 branch"),
        1..=  100 => {
            println!("Between 1 and 100 branch");
            println!("some more stuff");
        }
        _ => println!("else branch"),
    }


    let var_from_match = match some_bool {true => 10, false => 20};
    println!("{}", var_from_match);

    let var_from_match2 = match some_int {
        0 => 0,
        1 | 2 => 100,
        _ => 200,
    };

    println!("{}", var_from_match2);
}