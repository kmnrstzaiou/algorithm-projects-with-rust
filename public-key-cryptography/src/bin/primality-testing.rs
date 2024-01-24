use public_key_cryptography::{get_i64, Prng};

const NUM_TESTS: i64 = 20;

fn main() {
    // Prepare a Prng.
    let mut prng = Prng::new();

    // Display the probability that a number is prime
    // if it passes all NUM_TESTS tests.
    let probability = (1.0 - 0.5f64.powi(NUM_TESTS as i32)) * 100.0;
    println!("Probability: {}%\n", probability);

    // Generate random primes.
    loop {
        // Get the number of digits.
        let num_digits = get_i64("# Digits (max 9): ");
        if num_digits < 1 {
            break;
        }

        // Calculate minimum and maximum values.
        let mut min = 10i64.pow((num_digits - 1) as u32);
        let max = 10 * min;
        if min == 1 {
            min = 2;
        } // 1 is not prime.

        // Find a prime.
        println!("Prime: {}", find_prime(&mut prng, min, max, NUM_TESTS));
    }
}

fn find_prime(prng: &mut Prng, min: i64, max: i64, num_tests: i64) -> i64 {
    loop {
        let p = prng.next_i64(min, max);
        if p % 2 == 0 {
            continue;
        }
        if is_probably_prime(prng, p, num_tests) {
            return p;
        }
    }
}

fn is_probably_prime(prng: &mut Prng, p: i64, num_tests: i64) -> bool {
    for _ in 0..num_tests {
        let n = prng.next_i64(2, p - 1);
        let result = fast_exp_mod(n, p - 1, p);
        if result != 1 {
            return false;
        }
    }
    true
}

fn fast_exp_mod(num: i64, pow: i64, modulus: i64) -> i64 {
    do_fast_exp_mod(num, pow, 1, modulus)
}

fn do_fast_exp_mod(num: i64, pow: i64, result: i64, modulus: i64) -> i64 {
    match pow {
        0 => result,
        p if p % 2 != 1 => do_fast_exp_mod(num * num % modulus, p / 2, result, modulus),
        p => do_fast_exp_mod(num * num % modulus, p / 2, result * num % modulus, modulus),
    }
}
