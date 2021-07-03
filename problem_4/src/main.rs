use std::collections::HashMap;

fn main() {
    println!("{}", largest_palindromic_product(999));
}

fn largest_palindromic_product(max_factor: u64) -> u64 {
    if max_factor <= 3 {
        return max_factor * max_factor;
    }
    let mut possible_factors = HashMap::new();
    possible_factors.insert(max_factor, max_factor);
    loop {
        let (high_factor, low_factor) = largest_possible_product(possible_factors.clone());
        possible_factors.insert(high_factor, low_factor - 1);
        possible_factors.entry(high_factor - 1).or_insert(high_factor - 1);
        if is_palindromic_number(low_factor * high_factor) {
            return low_factor * high_factor;
        }
    };
}

fn largest_possible_product(possible_factors: HashMap<u64, u64>) -> (u64, u64) {
    let mut largest = (0, 0);
    for (key, value) in possible_factors {
        if key * value > largest.0 * largest.1 {
            largest = (key, value);
        }
    }
    largest
}

fn is_palindromic_number(n: u64) -> bool {
    let number = n.to_string();
    let len = number.len();
    let first_half_index = len / 2;
    let second_half_index = first_half_index + (len % 2);
    let first_half = &number[..first_half_index];
    let second_half = &number[second_half_index..].chars().rev().collect::<String>();
    first_half == second_half
}