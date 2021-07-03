use std::option::Option;
use std::option::Option::{Some, None};

fn main() {
    let number = 600851475143;
    let result = largest_prime_factor(number);
    match result {
        Some(n) => println!("Largest prime of {}: {}", number, n),
        None    => println!("{} is prime", number)
    }
}

fn largest_prime_factor(product: u64) -> Option<u64>{
    if product < 2 {
        return None
    }
    let mut i = 2;
    let mut largest_prime_factor = product;
    while i * i <= largest_prime_factor {
        while largest_prime_factor % i == 0 && largest_prime_factor != i {
            largest_prime_factor /= i;
        }
        i += 1
    }
    Some(largest_prime_factor)
}