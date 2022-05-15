fn main() {
    println!("Rust learning");
    // mutable
    println!("----------");
    println!(">> mutable / immutable");
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    // immutable
    println!(">> immutable");
    let y = 5;
    println!("The value of y is: {}", x);
    y = 6;
    println!("The value of y is: {}", x);
    println!("----------");
}
