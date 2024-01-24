use public_key_cryptography::get_i64;

fn main() {
    let a = get_i64("A: ");
    let b = get_i64("B: ");
    println!("GCD({}, {}) = {}", a, b, gcd(a, b));
    println!("LCM({}, {}) = {}", a, b, lcm(a, b));
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}
