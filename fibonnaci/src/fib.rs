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
    println!("Números de Fibonnaci:");
    io::stdin()
        .read_line(&mut guess)
        .expect("Falha ao ler");

    let guess: u128 = match guess.trim().parse() {
        Ok(num) => num,
        _ => return,
    };

    let old_time: DateTime<Utc> = Utc::now();

    println!("Número de Fibonnaci: {}", fibonacci(guess, Zero::zero(), One::one()));
    let duration = Utc::now().signed_duration_since(old_time);
}
