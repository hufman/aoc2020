extern crate uom;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.split("\n\n").map(|s| s.to_owned()).collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(lines: &[String]) -> i32 {
    let cards = lines.iter().map(|s| parse_card(s));
    cards.filter(|c| c.is_valid())
         .count() as i32
}

#[derive(Debug)]
struct Card {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>
}
impl Card {
    fn is_valid(self: &Self) -> bool {
        self.byr.is_some() &&
        self.iyr.is_some() &&
        self.eyr.is_some() &&
        self.hgt.is_some() &&
        self.hcl.is_some() &&
        self.ecl.is_some() &&
        self.pid.is_some()
    }
}

fn parse_card(line: &str) -> Card {
    //println!("Parsing {:}", line);
    let mut card = Card{byr: None, iyr: None, eyr: None, hgt: None, hcl: None, ecl: None, pid: None, cid: None};
    for field in line.split_whitespace() {
        let pieces: Vec<&str> = field.split(':').collect();
        if pieces.len() == 2 {
            let value = pieces[1].to_owned();
            match pieces[0] {
                "byr" => card.byr = Some(value),
                "iyr" => card.iyr = Some(value),
                "eyr" => card.eyr = Some(value),
                "hgt" => card.hgt = Some(value),
                "hcl" => card.hcl = Some(value),
                "ecl" => card.ecl = Some(value),
                "pid" => card.pid = Some(value),
                "cid" => card.cid = Some(value),
                _ => println!("Failed to parse {:}", field)
            }
        }
    }
    //println!("Into {:?}", card);
    card
}

