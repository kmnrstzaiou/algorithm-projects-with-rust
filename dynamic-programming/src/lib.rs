use std::time::{Instant, SystemTime, UNIX_EPOCH};

// ************
// *** Prng ***
// ************
#[derive(Default)]
pub struct Prng {
    pub seed: u32,
}

impl Prng {
    pub fn new() -> Self {
        let mut prng = Self::default();
        prng.randomize();
        prng
    }

    pub fn randomize(&mut self) {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        self.seed = millis as u32;
    }

    // Return a pseudorandom value in the range [0, 2147483647].
    fn next_u32(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.seed %= 1 << 31;
        self.seed
    }

    // Return a pseudorandom value in the range [0.0, 1.0).
    fn next_f64(&mut self) -> f64 {
        let f = self.next_u32() as f64;
        f / (2147483647.0 + 1.0)
    }

    // Return a pseudorandom value in the range [min, max).
    pub fn next_i32(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min) as f64;
        let result = min as f64 + range * self.next_f64();
        result as i32
    }
}

pub struct Item {
    pub value: i32,
    pub weight: i32,
    pub is_selected: bool,
}

// Make some random items.
pub fn make_items(
    prng: &mut Prng,
    num_items: i32,
    min_value: i32,
    max_value: i32,
    min_weight: i32,
    max_weight: i32,
) -> Vec<Item> {
    let mut items: Vec<Item> = Vec::with_capacity(num_items as usize);
    for _ in 0..num_items {
        let item = Item {
            value: prng.next_i32(min_value, max_value),
            weight: prng.next_i32(min_weight, max_weight),
            is_selected: false,
        };
        items.push(item);
    }
    items
}

// Return a copy of the items.
pub fn copy_items(items: &mut Vec<Item>) -> Vec<Item> {
    let mut new_items: Vec<Item> = Vec::with_capacity(items.len());
    for item in items {
        let new_item = Item {
            value: item.value,
            weight: item.weight,
            is_selected: item.is_selected,
        };
        new_items.push(new_item);
    }
    new_items
}

// Return the total value of the items.
// If add_all is true, add up all items.
// If add_all is false, only add up the selected items.
pub fn sum_values(items: &mut [Item], add_all: bool) -> i32 {
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

// Return the total weight of the items.
// If add_all is false, only add up the selected items.
// If add_all is true, add up all items.
pub fn sum_weights(items: &mut [Item], add_all: bool) -> i32 {
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

// Return the value of this solution.
// If the solution is too heavy, return -1, so we prefer an empty solution.
pub fn solution_value(items: &mut [Item], allowed_weight: i32) -> i32 {
    // If the solution's total weight > allowed_weight,
    // return -1 so even an empty solution is better.
    if sum_weights(items, false) > allowed_weight {
        return -1;
    }

    // Return the sum of the selected values.
    sum_values(items, false)
}

// Print the selected items.
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

// Run the algorithm. Display the elapsed time and solution.
type AlgFn = dyn Fn(&mut Vec<Item>, i32) -> (Vec<Item>, i32, i32);

pub fn run_algorithm(alg: &AlgFn, items: &mut Vec<Item>, allowed_weight: i32) {
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
