use dynamic_programming::{
    copy_items, make_items, run_algorithm, sum_values, sum_weights, Item, Prng,
};

const NUM_ITEMS: i32 = 35; // A reasonable value for branch and bound.

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

    // Branch and Bound
    if NUM_ITEMS > 40 {
        // Only run branch and bound if num_items is small enough.
        println!("Too many items for branch and bound\n");
    } else {
        println!("*** Branch and Bound ***");
        run_algorithm(&branch_and_bound, &mut items, allowed_weight);
    }
}

// Recursively assign values in or out of the solution.
// Return the best assignment, value of that assignment,
// and the number of function calls we made.
fn branch_and_bound(items: &mut Vec<Item>, allowed_weight: i32) -> (Vec<Item>, i32, i32) {
    do_branch_and_bound(
        &mut copy_items(items),
        allowed_weight,
        0,
        0,
        0,
        0,
        sum_values(items, true),
    )
}

fn do_branch_and_bound(
    items: &mut Vec<Item>,
    allowed_weight: i32,
    next_index: i32,
    best_value: i32,
    current_value: i32,
    current_weight: i32,
    remaining_value: i32,
) -> (Vec<Item>, i32, i32) {
    let copied_items = copy_items(items);
    if next_index == NUM_ITEMS {
        return (copied_items, current_value, 1);
    }
    if current_value + remaining_value <= best_value {
        return (vec![], 0, 1);
    }
    let next = &copied_items[next_index as usize];
    let (items1, total1, calls1) = if current_weight + next.weight <= allowed_weight {
        items[next_index as usize].is_selected = true;
        do_branch_and_bound(
            items,
            allowed_weight,
            next_index + 1,
            best_value,
            current_value + next.value,
            current_weight + next.weight,
            remaining_value - next.value,
        )
    } else {
        (vec![], 0, 1)
    };
    let (items2, total2, calls2) = if current_value + remaining_value - next.value > best_value {
        items[next_index as usize].is_selected = false;
        do_branch_and_bound(
            items,
            allowed_weight,
            next_index + 1,
            if total1 > best_value {
                total1
            } else {
                best_value
            },
            current_value,
            current_weight,
            remaining_value - next.value,
        )
    } else {
        (vec![], 0, 1)
    };
    let calls = calls1 + calls2 + 1;
    if total1 > total2 {
        (items1, total1, calls)
    } else {
        (items2, total2, calls)
    }
}
