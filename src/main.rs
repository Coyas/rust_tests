
fn main() {
    // cargo é awesome
    /**
     * signed: i8, i16, i32, i64, i128
     * unsigned: u8, u16, u32, u64, u128
     * architecture dependent: isize, usize
     * default: i32 (+/- 2Billion)
     * 
     * Floats
     * f32, f64
     * default is f64
     * 
     * char
     * 4 bits
     */

    let  x: i8 = 2;
    println!("valor de x: {}", x);
    let sum = x + 120;
    println!("valor de x: {}", sum);

    let num: i128 = 10;
    println!("num is {}", num);
    println!("Min i128 is {}", std::i128::MIN);
    println!("Max i128 is {}", std::i128::MAX);

    let data: f32 = 10.2154;
    println!("data is {}", data);
    println!("f32 max lengh is {}", std::f32::MAX);
    println!("f32 min lengh is {}", std::f32::MIN);

    let car: char = 'a';
    println!("is data: {}", car);
}
