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
    // The least amount of fuel aligns the crabs to the mode.
    let crab_count: HashMap<i32, usize> = crabs.iter().fold(HashMap::new(), |mut counts, crab| {
        *counts.entry(*crab).or_default() += 1;
        counts
    });

    let mode = crab_count.iter()
        .max_by_key(|(_, count)| *count)
        .map(|(crab, _)| *crab)
        .unwrap();

    // Calculate the fuel required to shift each crab.
    crabs.iter().map(|&crab| (crab - mode).abs()).sum()
}

// 16,1,2,0,4,2,7,1,2,14 - mode is 2, median is 49/10 = 4.9
//0,10 - any number in range is 10
//0,1,10 - any number is 11
//0,1,1,10 - 1 is 11, any other number is 12

#[test]
fn test_align_fuel() {
    assert_eq!(37, align_fuel(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]));
}