#![feature(try_blocks)]
#![feature(control_flow_enum)]

use std::ops::ControlFlow;

use sorting_and_searching::{check_sorted, get_i32, make_random_vec, print_vec};

fn main() {
    let num_items = get_i32("# of items? ");
    let max = get_i32("max number? ");
    let mut vec = make_random_vec(num_items, max);
    print_vec(&vec, num_items);
    bubble_sort(&mut vec);
    check_sorted(&vec);
}

fn bubble_sort(vec: &mut [i32]) {
    let n = vec.len();
    (0..n).try_for_each(|i| try {
        let swapped = (0..n - i - 1).fold(false, |swapped, j| {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
                true
            } else {
                swapped
            }
        });
        if !swapped {
            return ControlFlow::BREAK;
        }
    });
}
