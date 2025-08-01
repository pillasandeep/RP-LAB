fn pass_by_value(mut a: i32) {
    a += 1;
    println!("Inside pass_by_value: a = {}", a);
}

fn pass_by_reference(b: &mut i32) {
    *b += 1;
    println!("Inside pass_by_reference: b = {}", b);
}

fn main() {
    let a = 10;
    pass_by_value(a);
    println!("After pass_by_value: a = {}", a); // a remains unchanged

    let mut b = 10;
    pass_by_reference(&mut b);
    println!("After pass_by_reference: b = {}", b); // b is modified
}
