use sorting_and_searching::{get_i32, make_random_vec, print_vec_up_to_40_entries};

fn main() {
    let num_items = get_i32("# Items: ");
    let max = get_i32("Max: ");
    let mut vec = make_random_vec(num_items, max);
    vec.sort();
    print_vec_up_to_40_entries(&vec, num_items);
    loop {
        let target = get_i32("Target (-1 to quit): ");
        if target == -1 {
            break;
        }
        let (index, examined) = binary_search(&vec, target);
        if index != -1 {
            println!("numbers[{index}] = {target}, {examined} tests")
        } else {
            println!("target {target} not found, {examined} tests")
        }
    }
}

fn binary_search(vec: &[i32], target: i32) -> (i32, i32) {
    _binary_search(vec, target, 0, vec.len() - 1, 0)
}

fn _binary_search(vec: &[i32], target: i32, min: usize, max: usize, examined: i32) -> (i32, i32) {
    if max < min {
        return (-1, examined);
    }
    let guess = (max + min) / 2;
    match vec[guess] - target {
        0 => (guess as i32, examined + 1),
        x if x < 0 => _binary_search(vec, target, guess + 1, max, examined + 1),
        _ => _binary_search(vec, target, min, guess - 1, examined + 1),
    }
}
