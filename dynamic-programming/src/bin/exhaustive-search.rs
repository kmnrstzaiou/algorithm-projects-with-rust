use dynamic_programming::{
    copy_items, make_items, run_algorithm, solution_value, sum_values, sum_weights, Item, Prng,
};

const NUM_ITEMS: i32 = 20; // A reasonable value for exhaustive search.

const MIN_VALUE: i32 = 1;
const MAX_VALUE: i32 = 10;
const MIN_WEIGHT: i32 = 4;
const MAX_WEIGHT: i32 = 10;

fn main() {
    // Prepare a Prng using the same seed each time.
    let mut prng = Prng { seed: 1337 };
    // prng.randomize();

    // Make some random items.
    let mut items = make_items(
        &mut prng, NUM_ITEMS, MIN_VALUE, MAX_VALUE, MIN_WEIGHT, MAX_WEIGHT,
    );
    let allowed_weight = sum_weights(&mut items, true) / 2;

    // Display basic parameters.
    println!("*** Parameters ***");
    println!("# items:        {}", NUM_ITEMS);
    println!("Total value:    {}", sum_values(&mut items, true));
    println!("Total weight:   {}", sum_weights(&mut items, true));
    println!("Allowed weight: {}", allowed_weight);
    println!();

    // Exhaustive search
    if NUM_ITEMS > 23 {
        // Only run exhaustive search if num_items is small enough.
        println!("Too many items for exhaustive search\n");
    } else {
        println!("*** Exhaustive Search ***");
        run_algorithm(&exhaustive_search, &mut items, allowed_weight);
    }
}

// Recursively assign values in or out of the solution.
// Return the best assignment, value of that assignment,
// and the number of function calls we made.
fn exhaustive_search(items: &mut Vec<Item>, allowed_weight: i32) -> (Vec<Item>, i32, i32) {
    do_exhaustive_search(items, allowed_weight, 0)
}

fn do_exhaustive_search(
    items: &mut Vec<Item>,
    allowed_weight: i32,
    next_index: i32,
) -> (Vec<Item>, i32, i32) {
    if next_index == NUM_ITEMS {
        return (copy_items(items), solution_value(items, allowed_weight), 1);
    }
    let mut copied_items = copy_items(items);
    items[next_index as usize].is_selected = true;
    if solution_value(items, allowed_weight) == -1 {
        return do_exhaustive_search(&mut copied_items, allowed_weight, next_index + 1);
    }
    let (items1, total1, calls1) = do_exhaustive_search(items, allowed_weight, next_index + 1);
    let (items2, total2, calls2) =
        do_exhaustive_search(&mut copied_items, allowed_weight, next_index + 1);
    let calls = calls1 + calls2 + 1;
    if total1 > total2 {
        (items1, total1, calls)
    } else {
        (items2, total2, calls)
    }
}
