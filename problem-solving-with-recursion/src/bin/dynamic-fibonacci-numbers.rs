use problem_solving_with_recursion::get_i64;

fn main() {
    println!("Enter -1 to exit\n");
    let prefilled_values = prefill_vector();
    let mut fill_on_the_fly_values = vec![0, 1];
    loop {
        let n = get_i64("N: ");
        if n < 0 {
            break;
        }
        println!("Prefilled: {}", prefilled_values[n as usize]);
        println!(
            "On the fly: {}",
            fibonacci_on_the_fly(&mut fill_on_the_fly_values, n)
        );
        println!("Bottom up: {}", fibonacci_bottom_up(n));
        println!();
    }
}

fn fibonacci_on_the_fly(values: &mut Vec<i64>, n: i64) -> i64 {
    if n < values.len() as i64 {
        values[n as usize]
    } else {
        let result = fibonacci_on_the_fly(values, n - 1) + fibonacci_on_the_fly(values, n - 2);
        values.push(result);
        result
    }
}

fn prefill_vector() -> Vec<i64> {
    (2..93).fold(vec![0, 1], |mut vec, i| {
        vec.push(vec[i - 1] + vec[i - 2]);
        vec
    })
}

fn fibonacci_bottom_up(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }

    let mut fib_i_minus_2 = 0i64;
    let mut fib_i_minus_1 = 1i64;
    let mut fib_i = fib_i_minus_1 + fib_i_minus_2;
    for _ in 1i64..n {
        // Calculate this value of fib_i.
        fib_i = fib_i_minus_1 + fib_i_minus_2;

        // Set fib_i_minus_2 and fib_i_minus_1 for the next value.
        fib_i_minus_2 = fib_i_minus_1;
        fib_i_minus_1 = fib_i;
    }
    return fib_i;
}
