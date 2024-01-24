use std::{
    fmt::Display,
    io::{self, Write},
    time::{SystemTime, UNIX_EPOCH},
};

pub fn get_i32(prompt: &str) -> i32 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    trimmed.parse().expect("Error parsing integer")
}

#[derive(Default)]
pub struct Prng {
    seed: u32,
}

impl Prng {
    pub fn new() -> Self {
        let mut prng = Self::default();
        prng.randomize();
        prng
    }

    fn randomize(&mut self) {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        self.seed = millis as u32;
    }

    // Return a pseudorandom value in the range [0, 2147483647].
    pub fn next_u32(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.seed %= 1 << 31;
        self.seed
    }

    // Return a pseudorandom value in the range [0.0, 1.0).
    pub fn next_f64(&mut self) -> f64 {
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

// Make a vector of random i32 values in the range [0 and max).
pub fn make_random_vec(num_items: i32, max: i32) -> Vec<i32> {
    // Prepare a Prng.
    let mut prng = Prng::new();

    let mut vec: Vec<i32> = Vec::with_capacity(num_items as usize);
    for _ in 0..num_items {
        vec.push(prng.next_i32(0, max));
    }
    vec
}

// Print at most num_items items.
pub fn print_vec<T: Display>(vec: &[T], num_items: i32) {
    let mut max = vec.len();
    if max > num_items as usize {
        max = num_items as usize;
    }

    let mut string = String::new();
    string.push('[');

    if max > 0 {
        string.push_str(&vec[0].to_string());
    }

    (1..max).for_each(|i| {
        string.push(' ');
        string.push_str(&vec[i].to_string());
    });
    string.push(']');
    println!("{string}");
}

pub fn print_vec_up_to_40_entries<T: Display>(vec: &[T], num_items: i32) {
    let mut max = vec.len();
    if max > num_items as usize {
        max = num_items as usize;
    }

    let mut string = String::new();
    string.push('[');

    if max > 0 {
        string.push_str(&vec[0].to_string());
    }

    (1..std::cmp::min(max, 41)).for_each(|i| {
        string.push(' ');
        string.push_str(&vec[i].to_string());
    });
    if max > 40 {
        string += "..";
    } else {
        string.push(']');
    }
    println!("{string}");
}

pub fn check_sorted<T: Ord>(vec: &[T]) {
    (0..(vec.len() - 1)).for_each(|i| assert!(vec[i] <= vec[i + 1]));
    print!("sorted");
}
