use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn solution() {
    let crabs = load("input/day7.txt");

    println!("Part 1: {}", align_fuel(&crabs, linear_fuel));
    println!("Part 2: {}", align_fuel(&crabs, expensive_fuel));
}

fn load(filename: &str) -> Vec<i32> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().next().unwrap().unwrap().split(',').map(|n| n.parse().unwrap()).collect()
}

/// Returns the least amount of fuel required to align all of the crabs.
fn align_fuel(crabs: &Vec<i32>, fuel: fn(i32, i32) -> i32) -> i32 {
    // Brute force - calculate the fuel at all of the positions between the crabs.
    let (min, max) = crabs.iter().fold((0, 0), |(min, max), &crab| {
        (min.min(crab), max.max(crab))
    });

    (min ..= max)
        .map(|target| crabs.iter().map(|&crab| fuel(crab, target)).sum())
        .min()
        .unwrap()
}

/// Linear fuel calculation.  Each step costs 1 fuel.
fn linear_fuel(crab: i32, target: i32) -> i32 {
    (crab - target).abs()
}

/// Expensive fuel calculation.  Each step costs one more than the last - 1 step costs 1 fuel,
/// 2 steps costs 3 fuel, 3 costs 6, etc.
fn expensive_fuel(crab: i32, target: i32) -> i32 {
    // cost is 1 + 2 + ... + n = n*(n+1) / 2, which is the nth triangle number.
    let n = (crab - target).abs();
    n * (n + 1) / 2
}

#[test]
fn test_align_fuel() {
    assert_eq!(37, align_fuel(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14], linear_fuel));
    assert_eq!(168, align_fuel(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14], expensive_fuel));
}