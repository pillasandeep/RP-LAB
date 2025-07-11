fn main() {
    let int_value: i32 = 42;
    let float_value = int_value as f64; 
    
    println!("Integer value: {}", int_value);
    println!("Cast to float: {}", float_value);

    let large_float: f64 = 3.14159;
    let small_int = large_float as i32;

    println!("Large float value: {}", large_float);
    println!("Cast to int: {}", small_int);

    let unsigned_value: u32 = 100;
    let signed_value = unsigned_value as i32; 

    println!("Unsigned integer value: {}", unsigned_value);
    println!("Cast to signed int: {}", signed_value);
}
