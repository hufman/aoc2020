use std::iter::Iterator;
use std::ops::Add;
use std::ops::Index;

// An (x,y) pair
#[derive(Copy, Clone)]
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
    //width: i32,
    //height: i32
}

// A field can contain a point
impl Field {
    fn contains(&self, point: &Point) -> bool {
        point.y >= 0 && point.y < self.rows.len() as i32
    }
}
// A field can be indexed by a point, returning the character there
impl  Index<&Point> for Field {
    type Output = str;
    fn index(&self, point: &Point) -> &Self::Output {
        let row: &String = &self.rows[point.y as usize];
        let mut x = point.x;
        while x < 0 {
            x = x + row.len() as i32;
        }
        let col = x as usize % row.len();
        &row[col..col+1]
    }
}


#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines()
         .map(|s| s.to_owned())
         .collect()
}


#[aoc(day3, part1)]
pub fn solve_part1(input: &[String]) -> u32 {
    let rows: Vec<String> = input.iter()
                                 .map(|s| s.to_owned())
                                 .collect();
    let field = Field{rows};

    // solve
    let path = FieldPath{field: &field, pos: Point{x:0, y:0}, direction: Point{x:3, y:1}};
    let steps = path.map(|p| field[&p].chars().nth(0).unwrap());
    let trees = steps.filter(|&c| c == '#');
    trees.count() as u32
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
        if self.field.contains(&self.pos) {
            let cur_pos = self.pos;
            self.pos = self.pos + self.direction;
            Some(cur_pos)
        } else { None }
    }
}
