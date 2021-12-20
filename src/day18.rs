use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::str::FromStr;

use itertools::Either;
use itertools::Either::{Left, Right};

#[allow(dead_code)]
pub fn solution() {
    let pairs = load("input/day18.txt");

    println!("Part 1: {}", sum(&pairs).magnitude());
}

/// Loads pairs from the given file, one per line.
fn load(filename: &str) -> Vec<Pair> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    f.lines()
        .flat_map(|line| line.unwrap().parse::<Pair>())
        .collect()
}

/// Sums all of the pairs, returning the magnitude of the resulting pair.
fn sum(pairs: &Vec<Pair>) -> Pair {
    let mut it = pairs.iter();
    let start = it.next().unwrap();

    it.fold(start.clone(), |acc, pair| acc + pair.clone())
}

#[derive(Debug, Eq, PartialEq, Clone)]
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

        let mut stack: Vec<Either<Box<Pair>, i64>> = Vec::new();

        for c in s.chars() {
            if let Some(num) = c.to_digit(10) {
                stack.push(Right(num as i64));
            } else if c == ']' {
                let (right, left) = (stack.pop().unwrap(), stack.pop().unwrap());

                let pair = Pair { left, right };
                stack.push(Left(Box::new(pair)));
            }
        }

        Ok(*stack.pop().unwrap().expect_left("Parsed a number instead of a pair."))
    }
}

impl Add for Pair {
    type Output = Pair;

    fn add(self, rhs: Self) -> Self::Output {
        // Adding pairs results in a new pair.
        let mut result = Pair::pairs(self, rhs);

        // Reduce the result as far as possible.
        while result.reduce() {}

        result
    }
}

impl Pair {
    /// Constructs a new pair with two pairs.
    fn pairs(left: Pair, right: Pair) -> Self {
        Pair {
            left: Left(Box::new(left)),
            right: Left(Box::new(right)),
        }
    }

    /// Constructs a new pair with two numbers.
    fn nums(left: i64, right: i64) -> Self {
        Pair {
            left: Right(left),
            right: Right(right),
        }
    }

    /// Reduces this pair, modifying it in place.  Returns true if the pair was reduced,
    /// or false if no more reductions can be made.
    fn reduce(mut self) -> bool {
        fn reduce_node(node: Either<Box<Pair>, i64>, depth: usize) -> Option<Either<Box<Pair>, i64>> {
            match node {
                // If a pair is nested inside 4 pairs, the pair explodes.  The pair's left value
                // is added to the first regular number to the left of the exploding pair,
                // and the pair's right number is added to the first regular number to the right
                // of the exploding pair.  The exploding pair is replaced with the regular number 0.
                // TODO: The next number could be at any level relative to the exploding pair.
                Left(p) if depth == 3 => {todo!()},
                Left(p) => {todo!()},
                // If a regular number is 10 or greater, the number splits into a new pair
                // where the left side is the number / 2 rounded down, and the right side
                // is the number / 2 rounded up.
                Right(n) if n >= 10 => {
                    Some(Left(Box::new(Pair::nums(n / 2, n + 1 / 2))))
                },
                Right(n) => None
            }
        }

        reduce_node(Left(Box::new(self)), 0).is_some()
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
fn sample_explode() {
    let mut start: Pair = "[[[[[9,8],1],2],3],4]".parse().unwrap();
    let expected: Pair = "[[[[0,9],2],3],4]".parse().unwrap();
    assert_eq!(expected, start.reduce());

    let mut start: Pair = "[7,[6,[5,[4,[3,2]]]]]".parse().unwrap();
    let expected: Pair = "[7,[6,[5,[7,0]]]]".parse().unwrap();
    assert_eq!(expected, start.reduce());

    let mut start: Pair = "[[6,[5,[4,[3,2]]]],1]".parse().unwrap();
    let expected: Pair = "[[6,[5,[7,0]]],3]".parse().unwrap();
    assert_eq!(expected, start.reduce());

    let mut start: Pair = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".parse().unwrap();
    let expected: Pair = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse().unwrap();
    assert_eq!(expected, start.reduce());

    let mut start: Pair = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse().unwrap();
    let expected: Pair = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".parse().unwrap();
    assert_eq!(expected, start.reduce());

    let mut start: Pair = "[[3,[2,[[4,1],[7,3]]],0]".parse().unwrap();
    let expected: Pair = "[[3,[6,[0,[8,3]]],0]".parse().unwrap();
    assert_eq!(expected, start.reduce());
}

#[test]
fn sample_split() {
    let mut start: Pair = "[10,0]".parse().unwrap();
    let mut expected: Pair = "[[5,5],0]".parse().unwrap();
    assert_eq!(expected, start.reduce());

    let mut start: Pair = "[11,0]".parse().unwrap();
    let mut expected: Pair = "[[5,6],0]".parse().unwrap();
    assert_eq!(expected, start.reduce());

    let mut start: Pair = "[9,12]".parse().unwrap();
    let mut expected: Pair = "[9,[6,6]]".parse().unwrap();
    assert_eq!(expected, start.reduce());
}

#[test]
fn sample_sums() {
    let expected: Pair = "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]".parse().unwrap();
    assert_eq!(expected, sum(&load("input/day18_sample.txt")));
    let expected: Pair = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".parse().unwrap();
    assert_eq!(expected, sum(&load("input/day18_sample2.txt")));
    let expected: Pair = "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]".parse().unwrap();
    assert_eq!(expected, sum(&load("input/day18_sample3.txt")));
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
