extern crate itertools;
use itertools::iproduct;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    println!("Hello, world!");
    let stdin = io::stdin();
    let reader = stdin.lock();
    let numbers: Vec<i32> = reader.lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap().parse::<i32>())
        .filter(|i| i.is_ok())
        .map(|i| i.unwrap())
        .collect();

    let triple = find_triple(2020, numbers);
    match triple {
        Some(p) => println!("Found answer: {:?} => {}", p, p.0 * p.1 * p.2),
        None => println!("No result found"),
    }
}

fn find_triple(desired_sum: i32, nums: Vec<i32>) -> Option<(i32, i32, i32)> {
    let known_nums: HashSet<i32> = nums.iter().copied().collect();
    return iproduct!(nums.iter(), nums.iter()).into_iter()
        .find(|p| known_nums.contains(&(desired_sum - (p.0 + p.1))))
        .map(|p| (*p.0, *p.1, desired_sum - (*p.0 + *p.1)));
}
