use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use itertools::Itertools;

#[allow(dead_code)]
pub fn solution() {
    let entries = load("input/day8.txt");

    println!("Part 1: {}", num_unique(&entries));
    println!("Part 2: {}", decode(&entries));
}

fn load(filename: &str) -> Vec<Entry> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    f.lines()
        .flat_map(|line| line.unwrap().parse::<Entry>())
        .collect()
}

/// Returns the number of times that unique digits (1, 4, 7, 8) appear in the output values.
fn num_unique(entries: &Vec<Entry>) -> usize {
    // 1 has 2 segments, 4 has 4, 7 has 3, and 8 has 7.
    let segment_counts = vec![2, 3, 4, 7].into_iter().collect::<HashSet<usize>>();

    entries.iter()
        .flat_map(|entry| entry.output.iter().filter(|output| segment_counts.contains(&output.len())))
        .count()
}

/// Solves the wire / segment connections, and returns the sum of all of the output values.
fn decode(entries: &Vec<Entry>) -> i32 {
    entries.iter().map(|entry| entry.solve()).sum()
}

#[derive(Debug)]
enum ParseErr {}

#[derive(Eq, PartialEq)]
struct Entry {
    signal: Vec<String>,
    output: Vec<String>,
}

impl Entry {
    /// Solves the wire / segment connection for this entry, and returns the output value.
    fn solve(&self) -> i32 {
        //   a
        // b   c
        //   d
        // e   f
        //   g

        // len | segments
        // 2   | 1
        // 3   | 7
        // 4   | 4
        // 5   | 2, 3, 5
        // 6   | 0, 6, 9
        // 7   | 8

        // Signals has all 10 digits represented - sort them by length.
        let sorted_signals = self.signal.iter()
            .map(|s| s.chars().sorted().join(""))
            .sorted_by_key(|s| s.len())
            .collect_vec();

        let mut numbers = HashMap::new();

        // Some lengths only have one number.
        numbers.insert(&sorted_signals[0], 1);
        numbers.insert(&sorted_signals[1], 7);
        numbers.insert(&sorted_signals[2], 4);
        numbers.insert(&sorted_signals[9], 8);

        let one_letters: HashSet<char> = sorted_signals[0].chars().to_owned().collect();
        let four_letters: HashSet<char> = sorted_signals[2].chars().to_owned().collect();

        // length 5 - 2, 3, 5
        for signal_index in 3 ..= 5 {
            let signal = &sorted_signals[signal_index];
            let signal_letters: HashSet<char> = signal.chars().to_owned().collect();

            // 3 has all of the segments of 1
            // 5 is missing 1 segment from 4
            // 2 is missing 2 segments from 4
            if signal_letters.is_superset(&one_letters) {
                numbers.insert(signal, 3);
            } else if four_letters.difference(&signal_letters).count() == 1 {
                numbers.insert(signal, 5);
            } else {
                numbers.insert(signal, 2);
            }
        }

        // length 6 - 0, 6, 9
        for signal_index in 6 ..= 8 {
            let signal = &sorted_signals[signal_index];
            let signal_letters: HashSet<char> = signal.chars().to_owned().collect();

            // 6 is missing 1 segment from 1.
            // 0 is missing 1 segment from 4.
            // 9 has all of the segments from 4.
            if one_letters.difference(&signal_letters).count() == 1 {
                numbers.insert(signal, 6);
            } else if four_letters.difference(&signal_letters).count() == 1 {
                numbers.insert(signal, 0);
            } else {
                numbers.insert(signal, 9);
            }
        }

        // Translate the output.
        let mut output = 0;
        for s in &self.output {
            let digit = s.chars().sorted().join("");

            output = output * 10 + numbers[&digit];
        }

        output
    }
}

impl FromStr for Entry {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Entry looks like this: signal | output, with illuminated wires separated by spaces.
        // acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf

        let mut parts = s.split(" | ")
            .map(|part| part.split(" ").map(|s| s.to_string()).collect::<Vec<String>>());

        let signal = parts.next().unwrap();
        let output = parts.next().unwrap();

        Ok(Entry { signal, output })
    }
}

impl Debug for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for s in &self.signal {
            write!(f, "{} ", s)?
        }

        write!(f, "|")?;

        for o in &self.output {
            write!(f, " {}", o)?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_entry() {
        let expected = Entry {
            signal: vec!["acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab"].into_iter().map(|s| s.to_string()).collect(),
            output: vec!["cdfeb", "fcadb", "cdfeb", "cdbaf"].into_iter().map(|s| s.to_string()).collect(),
        };
        let parsed = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf".parse().unwrap();

        assert_eq!(expected, parsed)
    }

    #[test]
    fn test_num_unique() {
        let data = test_data();
        assert_eq!(26, num_unique(&data))
    }

    #[test]
    fn test_decode() {
        assert_eq!(61229, decode(&test_data()));
    }

    fn test_data<'a>() -> Vec<Entry> {
        vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
        ].iter()
            .map(|s| s.parse().unwrap())
            .collect()
    }
}