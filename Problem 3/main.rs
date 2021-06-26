use std::option::Option;
use std::option::Option::{Some, None};
use std::vec::Vec;

fn main() {
    let number = 600851475143;
    let result = largest_prime_factor(number);
    match result {
        Some(n) => println!("Largest prime of {}: {}", number, n),
        None    => println!("{} is prime", number)
    }
}

fn largest_prime_factor(product: u64) -> Option<u64> {
    let potential_prime_factors = potential_prime_factors_reversed(product);
    for prime in potential_prime_factors {
        if product % prime == 0 {
            return Some(prime);
        }
    }
    None
}

fn potential_prime_factors_reversed(product: u64) -> Vec<u64> {
    let mut found_primes = vec![2];
    let square_root_of_product = (product as f32).sqrt() as u64 + 1;
    for n in 3..square_root_of_product {
        let mut is_prime = true;
        for prime in &found_primes {
            if n % prime == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            found_primes.push(n);
        }
    }
    found_primes.reverse();
    found_primes
}