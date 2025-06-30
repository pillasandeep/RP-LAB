fn main() {
    let n = 5;

    for i in 1..=n {
        for _ in 0..i {
            print!("{}", i);
        }
        println!(); 
    }
}
