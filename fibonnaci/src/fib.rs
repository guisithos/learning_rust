extern crate num_bigint as bigint;
extern crate num_traits;
use bigint::BigUint;
use chrono::prelude::{DateTime, Utc};
use num_traits::{One, Zero};
use std::io;

fn fibonacci(number: u128, acc: BigUint, current: BigUint) -> BigUint {
    if number == 0 {
        return acc;
    } else {
        return fibonacci(number - 1, &acc + current, acc);
    }
}
fn main() {
    let mut guess = String::new();
    println!("Fibonacci number:");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u128 = match guess.trim().parse() {
        Ok(num) => num,
        _ => return,
    };

    let old_time: DateTime<Utc> = Utc::now();

    println!("fibonacci number: {}", fibonacci(guess, Zero::zero(), One::one()));
    let duration = Utc::now().signed_duration_since(old_time);

    println!(
        "{} µs",
        match duration.num_microseconds() {
            Some(value) => value,
            _ => 0,
        }
    );
}
