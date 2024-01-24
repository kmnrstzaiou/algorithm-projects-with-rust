use sorting_and_searching::{check_sorted, get_i32, make_random_vec, print_vec};

fn main() {
    let num_items = get_i32("# of items? ");
    let max = get_i32("max number? ");
    let mut vec = make_random_vec(num_items, max);
    print_vec(&vec, num_items);
    quicksort(&mut vec, 0, num_items - 1);
    check_sorted(&vec);
}

fn quicksort(vec: &mut [i32], lo: i32, hi: i32) {
    if lo >= hi || lo < 0 {
        return;
    }
    let p = partition(vec, lo, hi);
    quicksort(vec, lo, p - 1);
    quicksort(vec, p + 1, hi)
}

fn partition(vec: &mut [i32], lo: i32, hi: i32) -> i32 {
    let pivot = vec[hi as usize];
    let i = (lo..hi).fold(lo - 1, |mut i, j| {
        if vec[j as usize] <= pivot {
            i += 1;
            vec.swap(i as usize, j as usize);
        }
        i
    }) + 1;
    vec.swap(i as usize, hi as usize);
    i
}
