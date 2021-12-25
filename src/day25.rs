use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn solution() {
    let mut map = Map::load("input/day25.txt");

    println!("Part 1: {}", map.steps());
}

enum Square {
    South, East, Empty,
}

impl Square {
    fn from_char(c: char) -> Self {
        match c {
            'v' => Square::South,
            '>' => Square::East,
            '.' => Square::Empty,
            _ => panic!("Invalid char: {}", c),
        }
    }
}

struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Point { row, col }
    }
}

struct Map {
    squares: Vec<Vec<Square>>,
}

impl Map {
    fn load(filename: &str) -> Self {
        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);

        let mut squares = Vec::new();
        for line in f.lines() {
            squares.push(line.unwrap().chars().map(|c| Square::from_char(c)).collect());
        }

        Map { squares }
    }

    /// Returns the number of steps until no sea cucumbers move.
    fn steps(&mut self) -> usize {
        let mut steps = 0;
        let mut moving = true;

        let rows = self.squares.len();
        let cols = self.squares[0].len();

        while moving {
            // east facing herd moves, then south-facing herd.  Cucumbers move into open spaces.

            // Check which east-facing cucumbers can move.
            let mut move_east = Vec::new();
            for row in 0..rows {
                for col in 0..cols {
                    match (&self.squares[row][col], &self.squares[row][(col + 1) % cols]) {
                        (Square::East, Square::Empty) => move_east.push(Point::new(row, col)),
                        (_, _) => {}
                    }
                }
            }

            // Move east-facing cucumbers.
            for p in &move_east {
                self.squares[p.row][p.col] = Square::Empty;
                self.squares[p.row][(p.col + 1) % cols] = Square::East;
            }

            // Check which south-facing cucumbers can move.
            let mut move_south = Vec::new();
            for row in 0..rows {
                for col in 0..cols {
                    match (&self.squares[row][col], &self.squares[(row + 1) % rows][col]) {
                        (Square::South, Square::Empty) => move_south.push(Point::new(row, col)),
                        (_, _) => {}
                    }
                }
            }

            // Move south-facing cucumbers.
            for p in &move_south {
                self.squares[p.row][p.col] = Square::Empty;
                self.squares[(p.row + 1) % rows][p.col] = Square::South;
            }

            steps += 1;
            moving = !move_east.is_empty() || !move_south.is_empty();
        }

        steps
    }
}

#[test]
fn test_sample() {
    let mut map = Map::load("input/day25_sample.txt");
    assert_eq!(58, map.steps());
}
