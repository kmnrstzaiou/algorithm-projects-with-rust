use std::time::Instant;

use dynamic_programming::Prng;

const NUM_ITEMS: i32 = 60; // A reasonable value for branch and bound.

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

    // Rod's technique
    if NUM_ITEMS > 200 {
        // Only run Rod's technique if num_items is small enough.
        println!("Too many items for Rod's technique\n");
    } else {
        println!("*** Rod's Technique ***");
        run_algorithm(&rods_technique_sorted, &mut items, allowed_weight);
    }
}

struct Item {
    id: i32,
    value: i32,
    weight: i32,
    is_selected: bool,
    blocked_by: i32,
    block_list: Vec<i32>,
}

type AlgFn = dyn Fn(&mut [Item], i32) -> (Vec<Item>, i32, i32);

fn run_algorithm(alg: &AlgFn, items: &mut [Item], allowed_weight: i32) {
    // Copy the items so the run isn't influenced by a previous run.
    let mut test_items = copy_items(items);

    let start = Instant::now();

    // Run the algorithm.
    let mut solution: Vec<Item>;
    let total_value: i32;
    let function_calls: i32;
    (solution, total_value, function_calls) = alg(&mut test_items, allowed_weight);

    let duration = start.elapsed();
    println!("Elapsed: {:?}", duration);

    print_selected(&mut solution);
    println!(
        "Value: {}, Weight: {}, Calls: {}",
        total_value,
        sum_weights(&mut solution, false),
        function_calls
    );
    println!();
}

fn print_selected(items: &mut [Item]) {
    let mut num_printed = 0;
    for (i, item) in items.iter().enumerate() {
        if item.is_selected {
            print!("{}({}, {}) ", i, item.value, item.weight)
        }
        num_printed += 1;
        if num_printed > 100 {
            println!("...");
            return;
        }
    }
    println!();
}

fn sum_values(items: &mut [Item], add_all: bool) -> i32 {
    return if add_all {
        items.iter().map(|item| item.value).sum()
    } else {
        items
            .iter()
            .filter(|item| item.is_selected)
            .map(|item| item.value)
            .sum()
    };
}

fn sum_weights(items: &mut [Item], add_all: bool) -> i32 {
    return if add_all {
        items.iter().map(|item| item.weight).sum()
    } else {
        items
            .iter()
            .filter(|item| item.is_selected)
            .map(|item| item.weight)
            .sum()
    };
}

fn make_items(
    prng: &mut Prng,
    num_items: i32,
    min_value: i32,
    max_value: i32,
    min_weight: i32,
    max_weight: i32,
) -> Vec<Item> {
    let mut items: Vec<Item> = Vec::with_capacity(num_items as usize);
    for i in 0..num_items {
        items.push(Item {
            id: i,
            value: prng.next_i32(min_value, max_value),
            weight: prng.next_i32(min_weight, max_weight),
            is_selected: false,
            blocked_by: -1,
            block_list: vec![],
        });
    }
    items
}

fn copy_items(items: &mut [Item]) -> Vec<Item> {
    let mut new_items: Vec<Item> = Vec::with_capacity(items.len());
    for item in items {
        new_items.push(Item {
            id: item.id,
            value: item.value,
            weight: item.weight,
            is_selected: item.is_selected,
            blocked_by: item.blocked_by,
            block_list: item.block_list.clone(),
        });
    }
    new_items
}

fn make_block_lists(items: &mut [Item]) {
    for i in 0..items.len() {
        items[i].block_list = vec![];
        for j in 0..items.len() {
            if i != j && items[i].value >= items[j].value && items[i].weight <= items[j].weight {
                items[i].block_list.push(j as i32);
            }
        }
    }
}

// Recursively assign values in or out of the solution.
// Return the best assignment, value of that assignment,
// and the number of function calls we made.
fn _rods_technique(items: &mut [Item], allowed_weight: i32) -> (Vec<Item>, i32, i32) {
    make_block_lists(items);
    do_rods_technique(
        &mut copy_items(items),
        allowed_weight,
        0,
        0,
        0,
        0,
        sum_values(items, true),
    )
}

fn rods_technique_sorted(items: &mut [Item], allowed_weight: i32) -> (Vec<Item>, i32, i32) {
    make_block_lists(items);
    items.sort_by(|a, b| b.block_list.len().cmp(&a.block_list.len()));
    for (i, item) in items.iter_mut().enumerate() {
        item.id = i as i32;
    }
    make_block_lists(items);
    do_rods_technique(
        &mut copy_items(items),
        allowed_weight,
        0,
        0,
        0,
        0,
        sum_values(items, true),
    )
}

fn do_rods_technique(
    items: &mut Vec<Item>,
    allowed_weight: i32,
    next_index: i32,
    best_value: i32,
    current_value: i32,
    current_weight: i32,
    remaining_value: i32,
) -> (Vec<Item>, i32, i32) {
    if next_index == NUM_ITEMS {
        return (copy_items(items), current_value, 1);
    }
    if current_value + remaining_value <= best_value {
        return (vec![], 0, 1);
    }
    let current_item = &copy_items(items)[next_index as usize];
    let (test1_solution, test1_value, test1_calls) = if current_item.blocked_by == -1
        && current_weight + current_item.weight <= allowed_weight
    {
        items[next_index as usize].is_selected = true;
        do_rods_technique(
            items,
            allowed_weight,
            next_index + 1,
            best_value,
            current_value + current_item.value,
            current_weight + current_item.weight,
            remaining_value - current_item.value,
        )
    } else {
        (vec![], 0, 1)
    };
    for id in current_item.block_list.clone() {
        if items[id as usize].blocked_by == -1 {
            items[id as usize].blocked_by = current_item.id;
        }
    }
    items[next_index as usize].is_selected = false;
    let (test2_solution, test2_value, test2_calls) = do_rods_technique(
        items,
        allowed_weight,
        next_index + 1,
        if test1_value > best_value {
            test1_value
        } else {
            best_value
        },
        current_value,
        current_weight,
        remaining_value - current_item.value,
    );
    for id in current_item.block_list.clone() {
        if items[id as usize].blocked_by == current_item.id {
            items[id as usize].blocked_by = -1;
        }
    }

    let calls = test1_calls + test2_calls + 1;
    if test1_value > test2_value {
        (test1_solution, test1_value, calls)
    } else {
        (test2_solution, test2_value, calls)
    }
}
