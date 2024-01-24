use public_key_cryptography::{get_i64, Prng};

fn main() {
    let mut prng = Prng::new();
    let p = find_prime(&mut prng, 100, 1000, 10);
    let q = find_prime(&mut prng, 100, 1000, 10);
    let n = p * q;
    let λ_n = totient(p, q);
    let e = random_exponent(&mut prng, λ_n);
    let d = inverse_mod(e, λ_n);
    println!("*** Public ***");
    println!("Public key modulus:    {n}");
    println!("Public key exponent e: {e}");
    println!();
    println!("*** Private ***");
    println!("Primes: {p}, {q}");
    println!("λ(n):   {}", λ_n);
    println!("d:      {d}");
    println!();
    loop {
        let message = get_i64("Message:    ");
        if message < 1 {
            break;
        }
        let encrypted = fast_exp_mod(message, e, n);
        let decrypted = fast_exp_mod(encrypted, d, n);
        println!("Ciphertext: {}", encrypted);
        println!("Plaintext:  {}", decrypted);
        println!();
    }
}

fn totient(p: i64, q: i64) -> i64 {
    (p - 1) * (q - 1) / gcd(p - 1, q - 1)
}

fn random_exponent(prng: &mut Prng, λ_n: i64) -> i64 {
    loop {
        let e = prng.next_i64(3, λ_n);
        if gcd(e, λ_n) == 1 {
            return e;
        }
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn inverse_mod(a: i64, m: i64) -> i64 {
    let mut t = 0;
    let mut new_t = 1;
    let mut r = m;
    let mut new_r = a;

    while new_r != 0 {
        let quotient = r / new_r;
        let (t_, new_t_) = (new_t, t - quotient * new_t);
        let (r_, new_r_) = (new_r, r - quotient * new_r);
        t = t_;
        new_t = new_t_;
        r = r_;
        new_r = new_r_;
    }

    if r > 1 {
        panic!("{} is not invertible mod {}", a, m);
    }
    if t < 0 {
        t + m
    } else {
        t
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
