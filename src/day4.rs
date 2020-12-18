extern crate regex;
use self::regex::Regex;

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
    byr: Option<i32>,
    iyr: Option<i32>,
    eyr: Option<i32>,
    hgt: Option<Height>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>
}
impl Card {
    fn is_valid(self: &Self) -> bool {
        lazy_static! {
            static ref ECL_MATCHER: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            static ref HCL_MATCHER: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            static ref PID_MATCHER: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
        }
        self.byr.as_ref().filter(|&n| &1920<=n && n<=&2002).is_some() &&
        self.iyr.as_ref().filter(|&n| &2010<=n && n<=&2020).is_some() &&
        self.eyr.as_ref().filter(|&n| &2020<=n && n<=&2030).is_some() &&
        self.hgt.as_ref().filter(|n| {
            match n {
                Height::CM(ref i) => &150 <= i && i <= &193,
                Height::IN(ref i) => &59 <= i && i <= &76,
            }}).is_some() &&
        self.hcl.as_ref().filter(|n| HCL_MATCHER.is_match(&n)).is_some() &&
        self.ecl.as_ref().filter(|n| ECL_MATCHER.is_match(&n)).is_some() &&
        self.pid.as_ref().filter(|n| PID_MATCHER.is_match(&n)).is_some()
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
                "byr" => card.byr = value.parse::<i32>().ok(),
                "iyr" => card.iyr = value.parse::<i32>().ok(),
                "eyr" => card.eyr = value.parse::<i32>().ok(),
                "hgt" => card.hgt = parse_height(pieces[1]),
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

#[derive(Clone, Copy, Debug)]
enum Height {
    IN(i32),
    CM(i32),
}
fn parse_height(height: &str) -> Option<Height> {
    lazy_static! {
        static ref HEIGHT_MATCHER: Regex = Regex::new(r"^([0-9]*)(cm|in)$").unwrap();
    }
    HEIGHT_MATCHER.captures(height).map(|m| {
        let size = m.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let unit = m.get(2).unwrap().as_str();
        match unit {
            "cm" => Some(Height::CM(size)),
            "in" => Some(Height::IN(size)),
            _ => None
        }
    }).flatten()
}
