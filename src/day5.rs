use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[allow(dead_code)]
pub fn solution() {
    let lines = load("input/day5.txt");

    println!("Part 1: {}", num_overlapping(&lines, |line| line.is_horizontal() || line.is_vertical()));
    println!("Part 2: {}", num_overlapping(&lines, |_| true));
}

/// Loads lines from the given file.
fn load(filename: &str) -> Vec<Line> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    f.lines()
        .flat_map(|line| line.unwrap().parse::<Line>())
        .collect()
}

/// Returns the number of points where at least lines overlap.
fn num_overlapping(lines: &Vec<Line>, keep: fn(&Line) -> bool) -> usize {
    let mut point_lines: HashMap<Point, usize> = HashMap::new();

    for line in lines {
        if keep(line) {
            for point in line.points() {
                *point_lines.entry(point).or_default() += 1
            }
        }
    }

    point_lines.values().filter(|count| **count >= 2).count()
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Line {
    from: Point,
    to: Point,
}

impl Line {
    fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        Line {
            from: Point::new(x1, y1),
            to: Point::new(x2, y2),
        }
    }

    /// Returns whether this line is horizontal.
    fn is_horizontal(&self) -> bool {
        self.from.y == self.to.y
    }

    /// Returns whether this line is vertical.
    fn is_vertical(&self) -> bool {
        self.from.x == self.to.x
    }

    /// Returns all of the points on this line.
    fn points(&self) -> Vec<Point> {
        let x_step = (self.to.x as i32 - self.from.x as i32).signum();
        let y_step = (self.to.y as i32 - self.from.y as i32).signum();
        let length = (self.from.x as i32 - self.to.x as i32).abs()
            .max((self.from.y as i32 - self.to.y as i32).abs());

        (0..=length as i32)
            .map(|i| Point::new(
                (self.from.x as i32 + i * x_step) as usize,
                (self.from.y as i32 + i * y_step) as usize,
            ))
            .collect()
    }
}

#[derive(Debug)]
struct ParseErr {}

impl FromStr for Line {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Line looks like '0,9 -> 5,9'
        let nums = s.split(" -> ")
            .flat_map(|p| p.split(','))
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        if nums.len() != 4 {
            Err(ParseErr {})
        } else {
            Ok(Line::new(nums[0], nums[1], nums[2], nums[3]))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let lines = load("input/day5_sample.txt");
        assert_eq!(5, num_overlapping(&lines, |line| line.is_horizontal() || line.is_vertical()));
        assert_eq!(12, num_overlapping(&lines, |line| true));
    }

    #[test]
    fn test_line_points() {
        assert_eq!(vec![Point::new(1, 1), Point::new(2, 2), Point::new(3, 3)], Line::new(1, 1, 3, 3).points());
        assert_eq!(vec![Point::new(9, 7), Point::new(8, 8), Point::new(7, 9)], Line::new(9, 7, 7, 9).points());
    }
}