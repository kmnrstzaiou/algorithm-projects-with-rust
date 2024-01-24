fn main() {
    (0..22).for_each(|n| println!("{}! = {}", n, factorial(n)));
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
