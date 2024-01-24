use std::{cmp::Ordering, fmt};

use sorting_and_searching::{
    check_sorted, get_i32, make_random_vec as _make_random_vec, print_vec,
};

fn main() {
    let num_items = get_i32("# of items? ");
    let max = get_i32("max number? ");
    let mut vec = make_random_vec(num_items, max);
    print_vec(&vec, num_items);
    let vec = counting_sort(&mut vec, max);
    print_vec(&vec, num_items);
    check_sorted(&vec)
}

fn counting_sort(vec: &mut [Customer], radix: i32) -> Vec<Customer> {
    let mut b = vec![Customer::default(); vec.len()];
    let mut c = vec
        .iter()
        .fold(vec![0; radix as usize], |mut counts, c| {
            counts[c.num_purchases as usize] += 1;
            counts
        })
        .iter()
        .scan(0, |cum, &i| {
            *cum += i;
            Some(*cum)
        })
        .collect::<Vec<usize>>();
    vec.reverse();
    vec.iter().for_each(|m| {
        c[m.num_purchases as usize] -= 1;
        b[c[m.num_purchases as usize]] = m.clone();
    });
    b
}

#[derive(Clone, Default, Eq)]
struct Customer {
    id: String,
    num_purchases: i32,
}

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.num_purchases)
    }
}

impl PartialEq<Self> for Customer {
    fn eq(&self, other: &Self) -> bool {
        self.num_purchases == other.num_purchases
    }
}

impl PartialOrd<Self> for Customer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Customer {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.num_purchases - other.num_purchases {
            0 => Ordering::Equal,
            x if x < 0 => Ordering::Less,
            _ => Ordering::Greater,
        }
    }
}

fn make_random_vec(num_items: i32, max: i32) -> Vec<Customer> {
    _make_random_vec(num_items, max)
        .into_iter()
        .zip(1..)
        .map(|(n, i)| Customer {
            id: format!("C{i}"),
            num_purchases: n,
        })
        .collect()
}
