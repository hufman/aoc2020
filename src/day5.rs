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

#[aoc(day5, part2)]
pub fn solve_part2(seats: &[u32]) -> u32 {
    let mut sorted_seats = seats.to_owned();
    sorted_seats.sort();
    // find the missing number between i-1 and i+1
    sorted_seats.windows(2).filter(|s| {
        s[0] == s[1] - 2
    }).nth(0).unwrap()[0] + 1
}
