use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use std::ops::Add;
use std::str::FromStr;
use std::string::ParseError;

#[allow(dead_code)]
pub fn solution() {
    let numbers = load("input/day18.txt");

    println!("Part 1: {}", sum(&numbers).magnitude());
    println!("Part 2: {}", largest_magnitude(&numbers));
}

/// Loads pairs from the given file, one per line.
fn load(filename: &str) -> Vec<Number> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    f.lines()
        .flat_map(|line| line.unwrap().parse::<Number>())
        .collect()
}

/// Returns the sum of all of the numbers.
fn sum(numbers: &Vec<Number>) -> Number {
    let mut it = numbers.iter();
    let start = it.next().unwrap();

    it.fold(start.clone(), |acc, number| acc + number.clone())
}

/// Returns the largest magnitude of any sum of two different numbers.
fn largest_magnitude(nums: &Vec<Number>) -> i32 {
    (0..nums.len())
        .flat_map(move |i| (0..nums.len()).map(move |j| (i, j)))
        .filter(|(i, j)| i != j)
        .map(|(i, j)| (nums[i].clone() + nums[j].clone()).magnitude())
        .max()
        .unwrap()
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Number {
    value: Vec<Element>
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Element {
    Open,
    Close,
    Number(i32),
}

impl Debug for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Open => write!(f, "["),
            Element::Close => write!(f, "]"),
            Element::Number(num) => write!(f, "{}", num),
        }
    }
}

impl FromStr for Number {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elements = Vec::new();
        let mut last_num = None;

        for c in s.chars() {
            if last_num.is_some() && (c == '[' || c == ']' || c == ',') {
                elements.push(Element::Number(last_num.unwrap()));
                last_num = None;
            }

            match c {
                '[' => elements.push(Element::Open),
                ']' => elements.push(Element::Close),
                value if c.is_numeric() => {
                    last_num = Some(last_num.unwrap_or(0) * 10 + value.to_digit(10).unwrap() as i32)
                },
                _ => {}
            }
        }

        Ok(Number { value: elements })
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        // Adding two Numbers results in a pair of numbers.
        let result_value = iter::once(Element::Open)
            .chain(self.value.into_iter())
            .chain(rhs.value.into_iter())
            .chain(iter::once(Element::Close))
            .collect();

        let mut result = Number { value: result_value };

        // Reduce the resulting pair as far as possible.
        while result.reduce() {}

        result
    }
}

impl Number {
    /// Reduces this pair, modifying it in place.  Returns true if the pair was reduced, or
    /// false if no more reductions can be made.
    fn reduce(&mut self) -> bool {
        // In a given reduce round, the number will either explode once, split once, or do nothing.

        // Check for explosions.
        if self.explode() {
            return true;
        }

        // Check for splits.
        if self.split() {
            return true;
        }

        false
    }

    /// Checks and performs the leftmost explosion.  Pairs nested more than four pairs deep
    /// explode, adding the left number to the next left-most number, the right number to the
    /// next rightmost number, and replacing the deeply nested pair with 0.
    fn explode(&mut self) -> bool {
        let mut depth = 0;
        let mut maybe_left_add_idx = None;
        for (i, &e) in self.value.iter().enumerate() {
            match e {
                Element::Open => {
                    depth += 1;

                    if depth == 5 {
                        // Found an exploding pair - next two elements should be numbers.
                        if let (Element::Number(left), Element::Number(right)) = (self.value[i + 1], self.value[i + 2]) {
                            // Add the left number into the next-closest left number.
                            if let Some(left_add_idx) = maybe_left_add_idx {
                                if let Element::Number(left_add_value) = self.value[left_add_idx] {
                                    self.value[left_add_idx] = Element::Number(left_add_value + left);
                                }
                            }

                            // Add the right number into the next-closest right number.
                            let maybe_right_add_idx = ((i + 3)..self.value.len())
                                .find(|r| if let Element::Number(_) = self.value[*r] { true } else { false });
                            if let Some(right_add_idx) = maybe_right_add_idx {
                                if let Element::Number(right_add_value) = self.value[right_add_idx] {
                                    self.value[right_add_idx] = Element::Number(right_add_value + right);
                                }
                            }

                            // Replace the pair with 0.
                            self.value.splice(i..i+4, [Element::Number(0)]);

                            return true;
                        } else {
                            panic!("Found a pair of non-numbers at depth 5 - index {} in {:?}", i, self);
                        }
                    }
                }
                Element::Close => depth -= 1,
                Element::Number(_) => maybe_left_add_idx = Some(i),
            }
        }

        // Didn't find a pair deep enough to explode.
        false
    }

    /// Checks and performs the leftmost split.  Any number greater than 10 is replaced with a pair
    /// where the the left is the number divided by two rounded down, and the right element is
    /// the number divided by two rounded up.
    fn split(&mut self) -> bool {
        for (i, &e) in self.value.iter().enumerate() {
            if let Element::Number(v) = e {
                if v >= 10 {
                    self.value.splice(i..i+1, [Element::Open, Element::Number(v / 2), Element::Number((v+1)/2), Element::Close]);
                    return true;
                }
            }
        }

        false
    }

    /// Returns the magnitude of this pair.
    /// The magnitude of a pair is `3 * left + 2 * right`, recursive.
    fn magnitude(&self) -> i32 {
        let mut stack = Vec::new();

        for e in &self.value {
            match e {
                Element::Open => {},
                Element::Number(num) => stack.push(*num),
                Element::Close => {
                    let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
                    stack.push(3 * a + 2 * b);
                }
            }
        }

        assert_eq!(stack.len(), 1, "Stack should have one element when computing magnitude");

        stack[0]
    }
}

#[test]
fn test_parse() {
    let actual = "[10,5]".parse().unwrap();
    let expected: Number = Number { value: vec![Element::Open, Element::Number(10), Element::Number(5), Element::Close]};
    assert_eq!(expected, actual);
}

#[test]
fn sample_explode() {
    let mut start: Number = "[[[[[9,8],1],2],3],4]".parse().unwrap();
    let expected: Number = "[[[[0,9],2],3],4]".parse().unwrap();
    start.reduce();
    assert_eq!(expected, start);

    let mut start: Number = "[7,[6,[5,[4,[3,2]]]]]".parse().unwrap();
    let expected: Number = "[7,[6,[5,[7,0]]]]".parse().unwrap();
    start.reduce();
    assert_eq!(expected, start);

    let mut start: Number = "[[6,[5,[4,[3,2]]]],1]".parse().unwrap();
    let expected: Number = "[[6,[5,[7,0]]],3]".parse().unwrap();
    start.reduce();
    assert_eq!(expected, start);

    let mut start: Number = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".parse().unwrap();
    let expected: Number = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse().unwrap();
    start.reduce();
    assert_eq!(expected, start);

    let mut start: Number = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse().unwrap();
    let expected: Number = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".parse().unwrap();
    start.reduce();
    assert_eq!(expected, start);

    let mut start: Number = "[[3,[2,[[4,1],[7,3]]],0]".parse().unwrap();
    let expected: Number = "[[3,[6,[0,[8,3]]],0]".parse().unwrap();
    start.reduce();
    assert_eq!(expected, start);
}

#[test]
fn sample_split() {
    let mut start: Number = "[10,0]".parse().unwrap();
    let expected: Number = "[[5,5],0]".parse().unwrap();
    start.reduce();
    assert_eq!(expected, start);

    let mut start: Number = "[11,0]".parse().unwrap();
    let expected: Number = "[[5,6],0]".parse().unwrap();
    start.reduce();
    assert_eq!(expected, start);

    let mut start: Number = "[9,12]".parse().unwrap();
    let expected: Number = "[9,[6,6]]".parse().unwrap();
    start.reduce();
    assert_eq!(expected, start);
}

#[test]
fn sample_sums() {
    let expected: Number = "[[[[1,1],[2,2]],[3,3]],[4,4]]".parse().unwrap();
    assert_eq!(expected, sum(&load("input/day18_sample.txt")));
    let expected: Number = "[[[[3,0],[5,3]],[4,4]],[5,5]]".parse().unwrap();
    assert_eq!(expected, sum(&load("input/day18_sample2.txt")));
    let expected: Number = "[[[[5,0],[7,4]],[5,5]],[6,6]]".parse().unwrap();
    assert_eq!(expected, sum(&load("input/day18_sample3.txt")));
    let expected: Number = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".parse().unwrap();
    assert_eq!(expected, sum(&load("input/day18_sample4.txt")));
    let expected: Number = "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]".parse().unwrap();
    assert_eq!(expected, sum(&load("input/day18_sample5.txt")));
}

#[test]
fn sample_magnitudes() {
    assert_eq!(143, "[[1,2],[[3,4],5]]".parse::<Number>().unwrap().magnitude());
    assert_eq!(1384, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse::<Number>().unwrap().magnitude());
    assert_eq!(445, "[[[[1,1],[2,2]],[3,3]],[4,4]]".parse::<Number>().unwrap().magnitude());
    assert_eq!(791, "[[[[3,0],[5,3]],[4,4]],[5,5]]".parse::<Number>().unwrap().magnitude());
    assert_eq!(1137, "[[[[5,0],[7,4]],[5,5]],[6,6]]".parse::<Number>().unwrap().magnitude());
    assert_eq!(3488, "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".parse::<Number>().unwrap().magnitude());
}

#[test]
fn sample_largest_magnitude() {
    assert_eq!(3993, largest_magnitude(&load("input/day18_sample5.txt")));
}
