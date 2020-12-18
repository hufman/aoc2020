extern crate itertools;
use self::itertools::Itertools;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.to_owned().split("\n\n").map(str::to_string).collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(blocks: &[String]) -> u32 {
    blocks.iter().map(|b: &String| {
        b.chars()
         .filter(|c| ('a'..='z').contains(c))
         .unique()
         .count() as u32
    }).sum()
}
