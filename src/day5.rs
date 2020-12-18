use std::cmp::max;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|line| {
        u32::from_str_radix(line.chars().map(|s| {
            match s {
                'B' => Some('1'),
                'F' => Some('0'),
                'L' => Some('0'),
                'R' => Some('1'),
                _ => None
            }.unwrap()
        }).collect::<String>().as_str(), 2).unwrap()
    }).collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(seats: &[u32]) -> u32 {
    seats.iter().fold(0, |a, &i| max(a, i))
}
