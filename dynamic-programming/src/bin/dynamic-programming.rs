use dynamic_programming::{
    copy_items, make_items, run_algorithm, sum_values, sum_weights, Item, Prng,
};

const NUM_ITEMS: i32 = 10000; // A reasonable value for dynamic programming.

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

    // Dynamic programming
    println!("*** Dynamic programming ***");
    run_algorithm(&dynamic_programming, &mut items, allowed_weight);
}

// Use dynamic programming to find a solution.
// Return the best assignment, value of that assignment,
// and the number of function calls we made.
fn dynamic_programming(items: &mut Vec<Item>, allowed_weight: i32) -> (Vec<Item>, i32, i32) {
    let mut solution_value: Vec<Vec<i32>> =
        Vec::with_capacity(NUM_ITEMS as usize * (allowed_weight as usize + 1));
    let mut prev_weight: Vec<Vec<i32>> =
        Vec::with_capacity(NUM_ITEMS as usize * (allowed_weight as usize + 1));
    solution_value.push(vec![0; allowed_weight as usize + 1]);
    prev_weight.push(vec![0; allowed_weight as usize + 1]);
    for w in 0..=allowed_weight as usize {
        if w >= items[0].weight as usize {
            solution_value[0][w] = items[0].value;
            prev_weight[0][w] = -1;
        } else {
            solution_value[0][w] = 0;
            prev_weight[0][w] = w as i32;
        }
    }
    for i in 1..NUM_ITEMS as usize {
        solution_value.push(vec![0; allowed_weight as usize + 1]);
        prev_weight.push(vec![0; allowed_weight as usize + 1]);
        for w in 0..=allowed_weight as usize {
            let value_without_i = solution_value[i - 1][w];
            let value_with_i = if items[i].weight <= w as i32 {
                solution_value[i - 1][w - items[i].weight as usize] + items[i].value
            } else {
                0
            };
            if value_without_i >= value_with_i {
                solution_value[i][w] = value_without_i;
                prev_weight[i][w] = w as i32;
            } else {
                solution_value[i][w] = value_with_i;
                prev_weight[i][w] = w as i32 - items[i].weight;
            }
        }
    }
    items.iter_mut().for_each(|item| item.is_selected = false);
    let mut back_w = allowed_weight;
    for back_i in (0..NUM_ITEMS as usize).rev() {
        let prev_w = prev_weight[back_i][back_w as usize];
        if prev_w != back_w {
            items[back_i].is_selected = true;
            back_w = prev_w;
        }
    }
    (
        copy_items(items),
        solution_value[NUM_ITEMS as usize - 1][allowed_weight as usize],
        1,
    )
}
