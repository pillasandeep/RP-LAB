fn main() {
    let x = 5;
    println!("Outside scope, x = {}", x);

    {
        let x = x + 5; // shadowing x
        println!("Inside inner scope, x = {}", x);
    }

    println!("Back to outer scope, x = {}", x);
}
