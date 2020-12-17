use std::iter::Iterator;
use std::ops::Add;
use std::ops::Index;

// An (x,y) pair
struct Point {
    x: i32,
    y: i32
}
impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

// A field on which to traverse
pub struct Field {
    rows: Vec<String>,
    width: i32,
    height: i32
}

// A field can contain a point
impl Field {
    fn contains(&self, point: Point) -> bool {
        point.y >= 0 && point.y <= self.height
    }
}
// A field can be indexed by a point, returning the character there
impl Index<Point> for Field {
    type Output = char;
    fn index(&self, point: Point) -> &Self::Output {
        let row = &self.rows[point.y as usize];
        let mut col = point.x as usize;
        while col < 0 {
            col = col + row.len();
        }
        col = col % row.len();
        &row.chars().nth(col).unwrap()
    }
}


#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> &Field {
    let rows: Vec<String> = input.lines()
                                 .map(|s| s.to_owned())
                                 .collect();
    let width = rows.iter()
                    .filter_map(|s| s.parse::<i32>().ok())
                    .fold(0, |sum, x| sum + x);
    let height = rows.len() as i32;
    &Field{rows, width, height}
}


#[aoc(day3, part1)]
pub fn solve_part1(input: &Field) -> u32 {
    let path = FieldPath{field: input, pos: Point{x:0, y:0}, direction: Point{x:3, y:1}};
    path.map(|p| input[p])
        .filter(|&c| c == '#')
        .count() as u32
}

// A path through a Field
// Iterating it moves the path forward
struct FieldPath<'a> {
    field: &'a Field,
    pos: Point,
    direction: Point,
}
impl Iterator for FieldPath<'_> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.field.contains(self.pos) {
            let curPos = self.pos;
            self.pos = self.pos + self.direction;
            Some(curPos)
        } else { None }
    }
}
