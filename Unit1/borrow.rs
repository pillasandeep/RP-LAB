fn main() {
    let x = 10;

    let y = &x;

    println!("Value of x via y (borrowed reference): {}", *y);

    // Mutable reference
    let mut z = 20;
    let w = &mut z;

    *w += 5;
    
    println!("Value of z after modifying via mutable reference: {}", z);
}
