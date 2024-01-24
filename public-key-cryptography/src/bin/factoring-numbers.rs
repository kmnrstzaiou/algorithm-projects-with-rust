use std::time::Instant;

use public_key_cryptography::{get_i64, print_numbers};

fn main() {
    let num = get_i64("Number of factor: ");
    let start0 = Instant::now();
    let primes = primes_up_to(num / 2);
    let duration0 = start0.elapsed();
    println!("Build sieve: {:?} seconds", duration0);

    // Find the factors the slow way.
    let start1 = Instant::now();
    let factors1 = find_factors(num);
    let duration1 = start1.elapsed();
    println!("find_factors: {:?} seconds", duration1);
    print_numbers(&factors1);
    println!("Product: {}", multiply_vector(&factors1));
    println!();

    // Use the Euler's sieve to find the factors.
    let start2 = Instant::now();
    let factors2 = find_factors_sieve(&primes, num);
    let duration2 = start2.elapsed();
    println!("find_factors_sieve: {:?} seconds", duration2);
    print_numbers(&factors2);
    println!("Product: {}", multiply_vector(&factors2));
    println!();
}

fn find_factors(num: i64) -> Vec<i64> {
    let mut n = num;
    let mut factors = vec![];
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 2;
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}

fn multiply_vector(v: &[i64]) -> i64 {
    v.iter().product::<i64>()
}

fn find_factors_sieve(primes: &[i64], num: i64) -> Vec<i64> {
    let mut factors = vec![];
    let mut i = 0;
    let mut n = num;
    while n > 1 {
        while n % primes[i] == 0 {
            factors.push(primes[i]);
            n /= primes[i];
        }
        i += 1;
    }
    factors
}

fn primes_up_to(max: i64) -> Vec<i64> {
    let mut sieve = vec![true; (max + 1) as usize];
    sieve[0] = false;
    sieve[1] = false;

    for i in 2..=max {
        if sieve[i as usize] {
            for j in (i * i..=max).step_by(i as usize) {
                sieve[j as usize] = false;
            }
        }
    }

    sieve
        .iter()
        .enumerate()
        .filter(|&(_, &is_prime)| is_prime)
        .map(|(i, _)| i as i64)
        .collect()
}
