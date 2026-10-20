use std::{cmp::Ordering, io}; //Import lib.
use rand::prelude::*; //import random lib 

fn main() {
    let secret_num = rand::rng().random_range(1..=100);  
    println!("Guessing game v1 on Rust");
    println!("Enter your number:");
    println!("Debug + [{secret_num}]");


    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to parse number");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, 
        };

        match input.cmp(&secret_num) {
            Ordering::Equal => {
                println!("Угадали!");
                break 
            },
            Ordering::Greater => println!("Загаданное число меньше"),
            Ordering::Less => println!("Загаданное число больше"),
        }
    }
}
