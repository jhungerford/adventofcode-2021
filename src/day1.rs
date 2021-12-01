use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solution() {
    let measurements = load_measurements("input/day1.txt");

    println!("Part 1: {}", num_increasing(&measurements));
    println!("Part 2: {}", num_increasing_windows(&measurements));
}

fn load_measurements(filename: &str) -> Vec<i32> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    f.lines()
        .flat_map(|line| line.unwrap().parse::<i32>())
        .collect()
}

/// Returns the number of measurements that increased from one to another.
/// For example, `1 3 2` would return 1 because 3 is the only increasing measurement.
fn num_increasing(measurements: &Vec<i32>) -> usize {
    measurements.iter()
        .zip(measurements.iter().skip(1))
        .filter(|(a, b)| a < b)
        .count()
}

/// Returns the number of times that the sum of three-measurement windows
/// increases over the measurements.
fn num_increasing_windows(measurements: &Vec<i32>) -> usize {
    let sums = measurements.windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<i32>>();

    sums.iter().zip(sums.iter().skip(1))
        .filter(|(a, b)| a < b)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_increasing() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, num_increasing(&measurements));
    }

    #[test]
    fn test_num_increasing_windows() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(5, num_increasing_windows(&measurements));
    }
}