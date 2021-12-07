use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn solution() {
    let crabs = load("input/day7.txt");

    println!("Part 1: {}", align_fuel(&crabs));
}

fn load(filename: &str) -> Vec<i32> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().next().unwrap().unwrap().split(',').map(|n| n.parse().unwrap()).collect()
}

/// Returns the least amount of fuel required to align all of the crabs.
fn align_fuel(crabs: &Vec<i32>) -> i32 {
    // Brute force - calculate the fuel at all of the positions between the crabs.
    let (min, max) = crabs.iter().fold((0, 0), |(min, max), &crab| {
        (min.min(crab), max.max(crab))
    });

    (min ..= max)
        .map(|target| crabs.iter().map(|&crab| (crab - target).abs()).sum())
        .min()
        .unwrap()

}

#[test]
fn test_align_fuel() {
    assert_eq!(37, align_fuel(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]));
}