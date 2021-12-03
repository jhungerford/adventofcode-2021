use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn solution() {
    let nums = load("input/day3.txt");

    println!("Part 1: {}", power_consumption(&nums));
    println!("Part 2: {}", life_support(&nums));
}

fn load(filename: &str) -> Vec<Vec<char>> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    f.lines()
        .map(|line| to_char_array(&line.unwrap()))
        .collect()
}

/// Calculates the power consumption of the given numbers, calculated by `gamma * epsilon`.
/// Gamma's digits are found by taking the most common bit in each position; epsilon's digits
/// are found by taking the least.  The first bit in each number is the rightmost one.
fn power_consumption(nums: &Vec<Vec<char>>) -> i32 {
    // Number of ones at each index.  Gamma and epsilon have the opposite endianness of nums,
    // but ones matches nums endianness.
    let mut ones = vec![0; nums[0].len()];

    // Count up the number of ones in each position
    for num in nums {
        for (i, &c) in num.iter().enumerate() {
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

/// Returns the life support rating for the given numbers, which is
/// `oxygen generator rating * co2 scrubber rating`.  Both ratings are determined by keeping
/// numbers that match a 'bit criteria'.  Oxygen keeps numbers with the most common value in
/// the current bit position, and co2 keeps numbers with the least common value.
fn life_support(nums: &Vec<Vec<char>>) -> i32 {
    let oxygen_rating = rating(nums.clone(), |bit, common| bit == common);
    let co2_rating = rating(nums.clone(), |bit, common| bit != common);

    oxygen_rating * co2_rating
}

/// Determines the rating for the given number, which is found by keeping numbers that match
/// the bit criteria for each bit.
fn rating(mut nums: Vec<Vec<char>>, keep: fn(char, char) -> bool) -> i32 {
    let num_bits = nums[0].len();

    for bit in 0..num_bits {
        let len = nums.len();
        if len == 1 {
            return to_i32(&nums[0]);
        }

        // Count the number of ones at the given bit.
        let mut num_ones = 0;
        for num in &nums {
            if num[bit] == '1' {
                num_ones += 1;
            }
        }

        // Determine the most popular number.  Nums can have an odd or even length,
        // and oxygen keeps 1's if there's a tie and co2 keeps 0's.
        let common = if len % 2 == 0 && num_ones == len / 2 {
            '1'
        } else if num_ones > len / 2 {
            '1'
        } else {
            '0'
        };

        // Keep numbers that match the bit criteria.
        nums = nums.into_iter()
            .filter(|num| keep(num[bit], common))
            .collect::<Vec<Vec<char>>>();
    }

    if nums.len() > 1 {
        panic!("Rating not determined after considering all bits.")
    }

    to_i32(&nums[0])
}

fn to_char_array(s: &str) -> Vec<char> {
    s.chars().collect::<Vec<char>>()
}

fn to_i32(s: &Vec<char>) -> i32 {
    let mut num = 0;
    for &c in s {
        num <<= 1;
        num += c as i32 - '0' as i32;
    }

    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_consumption() {
        let nums = test_nums();
        assert_eq!(198, power_consumption(&nums));
    }

    #[test]
    fn test_life_support() {
        let nums = test_nums();
        assert_eq!(230, life_support(&nums));
    }

    fn test_nums() -> Vec<Vec<char>> {
        vec![
            to_char_array("00100"),
            to_char_array("11110"),
            to_char_array("10110"),
            to_char_array("10111"),
            to_char_array("10101"),
            to_char_array("01111"),
            to_char_array("00111"),
            to_char_array("11100"),
            to_char_array("10000"),
            to_char_array("11001"),
            to_char_array("00010"),
            to_char_array("01010"),
        ]
    }
}