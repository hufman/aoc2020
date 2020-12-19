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

#[aoc(day6, part2)]
pub fn solve_part2(blocks: &[String]) -> u32 {
    blocks.iter().map(|b: &String| {
        let responses = b.lines()
         .map(|l| l.chars().collect())
         .fold1(|l: Vec<char>, r: Vec<char>| common_chars(&l, &r)).unwrap();
        println!("{:?} -> {}", responses, responses.len());
        responses
         .len() as u32
    }).sum()
}

fn common_chars(l: &Vec<char>, r: &Vec<char>) -> Vec<char> {
    // find the common elements between two lists of chars
    l.iter().sorted().merge(r.iter().sorted())
     .tuple_windows()
     .filter_map(|(&l, &r)| if l == r { Some(l) } else { None })
     .collect()
}
