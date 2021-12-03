use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn solution() {
    let nums = load("input/day3.txt");

    println!("Part 1: {}", power_consumption(&nums));
}

fn load(filename: &str) -> Vec<String> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    f.lines()
        .map(|line| line.unwrap())
        .collect()
}

/// Calculates the power consumption of the given numbers, calculated by `gamma * epsilon`.
/// Gamma's digits are found by taking the most common bit in each position; epsilon's digits
/// are found by taking the least.  The first bit in each number is the rightmost one.
fn power_consumption(nums: &Vec<String>) -> i32 {
    // Number of ones at each index.  Gamma and epsilon have the opposite endianness of nums,
    // but ones matches nums endianness.
    let mut ones = vec![0; nums[0].len()];

    // Count up the number of ones in each position
    for num in nums {
        for (i, c) in num.chars().enumerate() {
            if c == '1' {
                ones[i] += 1;
            }
        }
    }

    // Convert ones into gamma and epsilon.
    // Gamma counts the most common value of each digit, and epsilon counts the least.
    let (mut gamma, mut epsilon) = (0, 0);
    for (i, &num_one) in ones.iter().enumerate() {
        gamma <<= 1;
        epsilon <<= 1;

        if num_one > nums.len() / 2 {
            // 1 is the most popular digit, so record a 1 in gamma.
            gamma += 1;
        } else if num_one < nums.len() / 2 {
            // 0 is the most popular digit, so record a 1 in epsilon.
            epsilon += 1;
        } else {
            panic!("Digit {} has an equal number of 0's and 1's.", i);
        }
    }

    gamma * epsilon
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_consumption() {
        let nums = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];

        assert_eq!(198, power_consumption(&nums));
    }
}