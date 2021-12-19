use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Sum;
use std::ops::Add;
use std::str::FromStr;

use itertools::Either;

#[allow(dead_code)]
pub fn solution() {
    let pairs = load("input/day18.txt");

    println!("Part 1: {}", sum(&pairs));
}

fn load(filename: &str) -> Vec<Pair> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    f.lines()
        .flat_map(|line| line.unwrap().parse::<Pair>())
        .collect()
}

/// Sums all of the pairs, returning the magnitude of the resulting pair.
fn sum(pairs: &Vec<Pair>) -> i64 {
    pairs.iter().sum::<&Pair>().magnitude()
}

#[derive(Debug, Eq, PartialEq)]
struct Pair {
    left: Either<Box<Pair>, i64>,
    right: Either<Box<Pair>, i64>,
}

#[derive(Debug)]
enum ParseErr {}

impl FromStr for Pair {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Pair looks like '[[1,2],3]', and can be recursive.
        todo!()
    }
}

impl Add for &Pair {
    type Output = Pair;

    fn add(self, rhs: Self) -> Self::Output {
        // let mut result = Pair::new()

        todo!()
    }
}

impl Sum for &Pair {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        todo!()
    }
}

impl Pair {
    /// Returns whether this pair can be reduced.
    fn can_reduce(&self) -> bool {
        todo!()
    }

    fn reduce(&mut self) {
        todo!()
    }

    /// Returns the magnitude of this pair.
    fn magnitude(&self) -> i64 {
        match (&self.left, &self.right) {
            (Either::Left(l), Either::Left(r)) => 3 * l.magnitude() + 2 * r.magnitude(),
            (Either::Left(l), Either::Right(r)) => 3 * l.magnitude() + 2 * r,
            (Either::Right(l), Either::Left(r)) => 3 * l + 2 * r.magnitude(),
            (Either::Right(l), Either::Right(r)) => 3 * l + 2 * r,
        }
    }
}

#[test]
fn sample_sums() {
    let expected: Pair = "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]".parse().unwrap();
    assert_eq!(expected, *load("input/day18_sample.txt").iter().sum::<&Pair>());
    let expected: Pair = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".parse().unwrap();
    assert_eq!(expected, *load("input/day18_sample2.txt").iter().sum::<&Pair>());
    let expected: Pair = "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]".parse().unwrap();
    assert_eq!(expected, *load("input/day18_sample3.txt").iter().sum::<&Pair>());
}

#[test]
fn sample_magnitudes() {
    assert_eq!(143, "[[1,2],[[3,4],5]]".parse::<Pair>().unwrap().magnitude());
    assert_eq!(1384, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse::<Pair>().unwrap().magnitude());
    assert_eq!(445, "[[[[1,1],[2,2]],[3,3]],[4,4]]".parse::<Pair>().unwrap().magnitude());
    assert_eq!(791, "[[[[3,0],[5,3]],[4,4]],[5,5]]".parse::<Pair>().unwrap().magnitude());
    assert_eq!(1137, "[[[[5,0],[7,4]],[5,5]],[6,6]]".parse::<Pair>().unwrap().magnitude());
    assert_eq!(3488, "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".parse::<Pair>().unwrap().magnitude());
}
