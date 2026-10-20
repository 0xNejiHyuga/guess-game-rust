use std::io; //Import lib.

fn main() {
    println!("Guessing game v1 on Rust");
    println!("Enter your number:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to parse number");

    println!("Ur choose: {input}")
}
