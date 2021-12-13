use std::collections::VecDeque;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use itertools::Itertools;

#[allow(dead_code)]
pub fn solution() {
    let mut paper = Paper::load("input/day13.txt");

    println!("Part 1: {}", paper.fold_once());
}

#[derive(Debug)]
enum ParseErr {}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Dot {
    x: i32,
    y: i32,
}

impl FromStr for Dot {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Dot looks like '6,10'
        let mut nums = s.split(',').map(|i| i.parse().unwrap());
        Ok(Dot {
            x: nums.next().unwrap(),
            y: nums.next().unwrap(),
        })
    }
}

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32),
}

impl FromStr for Fold {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Fold looks like 'fold along y=7'
        let eq = s.find("=").unwrap();

        let index = s[eq+1..].parse().unwrap();

        if s.contains('x') {
            Ok(Fold::X(index))
        } else {
            Ok(Fold::Y(index))
        }
    }
}

#[derive(Debug)]
struct Paper {
    dots: Vec<Dot>,
    folds: VecDeque<Fold>,
}

impl Paper {
    /// Loads paper from the given file.  The file contains points like '6,10' where dots are
    /// visible, followed by fold instructions like 'fold along y=7'.
    fn load(filename: &str) -> Self {
        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);
        let lines = f.lines();

        let mut dots = Vec::new();
        let mut folds = VecDeque::new();
        let mut parsing_dots = true;

        for maybe_line in lines {
            let line = maybe_line.unwrap();
            if line.is_empty() {
                parsing_dots = false;
            } else if parsing_dots {
                dots.push(line.parse().unwrap());
            } else {
                folds.push_back(line.parse().unwrap());
            }
        }

        Paper { dots, folds }
    }

    /// Follows the next fold instruction, returning the number of dots that are visible.
    fn fold_once(&mut self) -> usize {
        // Fold the paper.
        match self.folds.pop_front().unwrap() {
            Fold::X(index) => {
                // all dots to the right of the index are folded left.
                for mut dot in &mut self.dots {
                    if dot.x > index {
                        dot.x -= 2 * (dot.x - index);
                    }
                }
            }
            Fold::Y(index) => {
                // all dots below the index are folded up.
                for mut dot in &mut self.dots {
                    if dot.y > index {
                        dot.y -= 2 * (dot.y - index);
                    }
                }
            }
        }

        // de-duplicate dots that overlapped.
        self.dots = self.dots.iter().unique().cloned().collect();

        // return the number of dots that are still visible.
        self.dots.len()
    }
}

#[test]
fn test_sample() {
    let mut paper = Paper::load("input/day13_sample.txt");
    assert_eq!(17, paper.fold_once());
}
