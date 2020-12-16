extern crate itertools;
use self::itertools::iproduct;
use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines()
        .map(|line| line.parse::<i32>())
        .filter(|i| i.is_ok())
        .map(|i| i.unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(numbers: &[i32]) -> Option<i32> {
    let pair = find_pair(2020, numbers);
    match pair {
        Some(p) => println!("Found answer: {:?} => {}", p, p.0 * p.1),
        None => println!("No result found"),
    }
    pair.map(|p| p.0 * p.1)
}

fn find_pair(desired_sum: i32, nums: &[i32]) -> Option<(i32, i32)> {
    let known_nums: HashSet<i32> = nums.iter().copied().collect();
    return nums
        .iter()
        .find(|&i| known_nums.contains(&(desired_sum - i)))
        .map(|&i| (i, desired_sum - i));
}

#[aoc(day1, part2)]
pub fn solve_part2(numbers: &[i32]) -> Option<i32> {
    let triple = find_triple(2020, numbers);
    match triple {
        Some(p) => println!("Found answer: {:?} => {}", p, p.0 * p.1 * p.2),
        None => println!("No result found"),
    }
    triple.map(|p| p.0 * p.1 * p.2)
}

fn find_triple(desired_sum: i32, nums: &[i32]) -> Option<(i32, i32, i32)> {
    let known_nums: HashSet<i32> = nums.iter().copied().collect();
    return iproduct!(nums.iter(), nums.iter()).into_iter()
        .find(|p| known_nums.contains(&(desired_sum - (p.0 + p.1))))
        .map(|p| (*p.0, *p.1, desired_sum - (*p.0 + *p.1)));
}
