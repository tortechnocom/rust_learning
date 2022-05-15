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
    // y = 6;
    println!("The value of y is: {}", x);
    println!("----------");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Constants: {}", THREE_HOURS_IN_SECONDS);
}
