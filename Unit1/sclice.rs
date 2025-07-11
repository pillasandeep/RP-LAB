fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let slice1 = &arr[1..3];
    println!("2nd and 3rd elements: {:?}", slice1);

    let slice2 = &arr[..3];
    println!("First 3 elements: {:?}", slice2);

    let slice3 = &arr[3..];
    println!("From 4th to end: {:?}", slice3);

    let slice4 = &arr[..];
    println!("Full array slice: {:?}", slice4);
}
