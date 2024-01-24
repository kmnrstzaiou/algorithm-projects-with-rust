use public_key_cryptography::{get_i64, print_numbers};

fn main() {
    let max = get_i64("Max: ");
    let sieve = sieve_of_eratosthenes(max as usize);
    if max < 1000 {
        print_sieve(&sieve);
    }

    let primes = sieve_to_primes(sieve);
    if max < 1000 {
        print_numbers(&primes);
    }
}

fn sieve_of_eratosthenes(max: usize) -> Vec<bool> {
    let mut sieve = vec![true; max + 1];
    sieve[0] = false;
    sieve[1] = false;

    for i in 2..=max {
        if sieve[i] {
            for j in (i * i..=max).step_by(i) {
                sieve[j] = false;
            }
        }
    }

    sieve
}

fn print_sieve(sieve: &[bool]) {
    print!("2 ");
    for (&is_prime, i) in sieve[3..].iter().zip(3..).step_by(2) {
        if is_prime {
            print!("{i} ");
        }
    }
    println!();
}

fn sieve_to_primes(sieve: Vec<bool>) -> Vec<i64> {
    sieve[3..]
        .iter()
        .zip(3..)
        .step_by(2)
        .filter(|(&is_prime, _)| is_prime)
        .fold(vec![2], |mut v, (_, i)| {
            v.push(i as i64);
            v
        })
}
