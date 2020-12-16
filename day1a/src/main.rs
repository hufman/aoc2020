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

    let pair = find_pair(2020, numbers);
    match pair {
        Some(p) => println!("Found answer: {:?} => {}", p, p.0 * p.1),
        None => println!("No result found"),
    }
}

fn find_pair(desired_sum: i32, nums: Vec<i32>) -> Option<(i32, i32)> {
    //let mut known_nums = HashSet::new();
    //nums.iter().for_each(|&i| known_nums.insert(i););
    let known_nums: HashSet<i32> = nums.iter().copied().collect();
    return nums
        .iter()
        .find(|&i| known_nums.contains(&(desired_sum - i)))
        .map(|&i| (i, desired_sum - i));
}
