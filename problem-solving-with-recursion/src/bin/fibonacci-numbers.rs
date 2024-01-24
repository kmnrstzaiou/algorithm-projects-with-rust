use problem_solving_with_recursion::get_i64;

fn main() {
    println!("Enter -1 to exit\n");
    loop {
        let n = get_i64("N: ");
        if n < 0 {
            break;
        }
        println!("fibonacci({}) = {}\n", n, fibonacci(n));
    }
}

fn fibonacci(n: i64) -> i64 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
