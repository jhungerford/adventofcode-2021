use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn solution() {
    let mut fish = Fish::load("input/day6.txt");

    println!("Part 1: {}", fish.tick_days(80));
    println!("Part 2: {}", fish.tick_days(256-80));
}

#[derive(Debug)]
struct Fish {
    /// Indexes are timers (0..=8), and values are the number
    /// of fish at each timer.
    counts: Vec<u64>
}

impl Fish {
    /// Constructs a new school of fish from the given timers.
    fn new(timers: Vec<u64>) -> Self {
        // Fish ages range from 0 ..= 8
        let mut counts = vec![0; 9];

        for timer in timers {
            counts[timer as usize] += 1;
        }

        Self { counts }
    }

    /// Loads fish from the given file, which contains a comma-separated
    /// list of fish timers on the first line.
    fn load(filename: &str) -> Self {
        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);
        let ages = f.lines().next().unwrap().unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        Fish::new(ages)
    }

    /// Ticks this school of lanternfish.  Fish decrement their
    /// timers every day.  Fish with a timer of 0 spawn a new fish
    /// at 8, and reset their timers to 6.
    fn tick(&mut self) {
        let mut new_counts = vec![0; self.counts.len()];

        // Fish at timer 0 spawn new fish at timer 8, and reset their timers to 6
        new_counts[6] = self.counts[0];
        new_counts[8] = self.counts[0];

        // Other fish decrement their counters.
        for i in 1..self.counts.len() {
            new_counts[i - 1] += self.counts[i];
        }

        // Save the updated counters.
        self.counts = new_counts;
    }

    /// Ticks the given number of days, returning the
    /// total number of lanternfish.
    fn tick_days(&mut self, days: usize) -> u64 {
        for _ in 0..days {
            self.tick();
        }

        self.total()
    }

    /// Returns the total number of lanternfish.
    fn total(&self) -> u64 {
        self.counts.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let mut fish = Fish::new(vec![3,4,3,1,2]);
        assert_eq!(26, fish.tick_days(18));
        assert_eq!(5934, fish.tick_days(80-18));
        assert_eq!(26984457539, fish.tick_days(256-80));
    }
}
