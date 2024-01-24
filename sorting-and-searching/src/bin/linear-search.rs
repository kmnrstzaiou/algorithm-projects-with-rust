use sorting_and_searching::{get_i32, make_random_vec, print_vec_up_to_40_entries};

fn main() {
    let num_items = get_i32("# Items: ");
    let max = get_i32("Max: ");
    let vec = make_random_vec(num_items, max);
    print_vec_up_to_40_entries(&vec, num_items);
    loop {
        let target = get_i32("Target (-1 to quit): ");
        if target == -1 {
            break;
        }
        let (index, examined) = linear_search(&vec, target);
        if index != -1 {
            println!("numbers[{index}] = {target}, {examined} tests")
        } else {
            println!("target {target} not found, {examined} tests")
        }
    }
}

fn linear_search(vec: &[i32], target: i32) -> (i32, i32) {
    match vec
        .iter()
        .zip(0..)
        .try_fold((-1, 0), |(index, examined), (&i, n)| {
            if i == target {
                Err((n, examined + 1))
            } else {
                Ok((index, examined + 1))
            }
        }) {
        Err((i, e)) => (i, e),
        Ok((i, e)) => (i, e),
    }
}
