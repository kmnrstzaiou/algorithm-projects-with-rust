use public_key_cryptography::get_i64;

fn main() {
    let num = get_i64("Number: ");
    let pow = get_i64("Power: ");
    println!(
        "{}^{} = {}({:?})",
        num,
        pow,
        fast_exp(num, pow),
        num.checked_pow(pow as u32)
    );
    let modulus = get_i64("Modulus: ");
    println!(
        "{}^{} mod {} = {}",
        num,
        pow,
        modulus,
        fast_exp_mod(num, pow, modulus)
    );
}

// Perform fast exponentiation.
fn fast_exp(num: i64, pow: i64) -> i64 {
    do_fast_exp(num, pow, 1)
}

fn do_fast_exp(num: i64, pow: i64, result: i64) -> i64 {
    if pow == 0 {
        return result;
    }
    match num.checked_mul(num) {
        Some(mul) if pow % 2 != 1 => do_fast_exp(mul, pow / 2, result),
        Some(mul) => do_fast_exp(mul, pow / 2, result * num),
        None => {
            println!("Overflow: {n} * {n}", n = num);
            -1
        }
    }
}

// Perform fast exponentiation in a modulus.
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
