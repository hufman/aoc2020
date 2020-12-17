use std::ops::Range;

pub struct PasswordRule {
    params: (usize, usize),
    character: char
}
pub struct Password {
   rule: PasswordRule,
   password: String
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
    input.lines().filter_map(|l| parse_password(l))
                 .collect()
}

fn parse_password(input: &str) -> Option<Password> {
    let fields: Vec<&str> = input.split(":").collect();
    if fields.len() == 2 {
        let rule = parse_password_rule(fields[0]);
        match rule {
            Some(rule) => Some(Password { rule: rule, password: (fields[1].to_string()) }),
            None => None
        }
    } else { None }
}

fn parse_password_rule(input: &str) -> Option<PasswordRule> {
    let fields: Vec<&str> = input.split(" ").collect();
    let range_split: Vec<usize> = fields[0].split("-")
                                         .filter_map(|s| s.parse::<usize>().ok())
                                         .collect();
    if fields.len() == 2 && range_split.len() == 2 {
        Some(PasswordRule { params: (range_split[0], range_split[1]), character: (fields[1].chars().nth(0).unwrap()) })
    } else { None }
}


#[aoc(day2, part1)]
pub fn solve_part1(input: &[Password]) -> u32 {
    input.into_iter()
         .filter(|p| test_password_part1(p))
         .count() as u32
}

fn test_password_part1(password: &Password) -> bool {
    let count = password.password.chars()
        .filter(|&c| c == password.rule.character)
        .count();
    let range = Range{start: password.rule.params.0, end: password.rule.params.1 + 1};
    range.contains(&count)
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Password]) -> u32 {
    input.into_iter()
         .filter(|p| test_password_part2(p))
         .count() as u32
}

fn test_password_part2(password: &Password) -> bool {
    let needed_char = password.rule.character;
    let params = password.rule.params;
    let chars: Vec<char> = password.password.chars().collect();
    let first = chars.get(params.0).map(|&c| c == needed_char).unwrap_or(false);
    let second = chars.get(params.1).map(|&c| c == needed_char).unwrap_or(false);
    first && !second || !first && second
}

