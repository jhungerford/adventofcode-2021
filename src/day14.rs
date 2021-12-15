use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn solution() {
    let poly = Polymerization::load("input/day14.txt");

    println!("Part 1: {}", poly.score(10));
    println!("Part 2: {}", poly.score(40));
}

#[derive(Debug)]
struct Polymerization {
    template: Vec<char>,
    rules: HashMap<(char, char), char>,
}

impl Polymerization {
    /// Loads a Polymerization from the given file, which contains a polymer template
    /// followed by rules.
    fn load(filename: &str) -> Self {
        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);
        let mut lines = f.lines();

        // Template.
        let template: Vec<char> = lines.next().unwrap().unwrap().chars().collect();

        // Blank line.
        let _ = lines.next();

        // Rules
        let mut rules = HashMap::new();
        for next_line in lines {
            // Rules look like 'NV -> S'
            let line = next_line.unwrap();
            let mut split = line.split(" -> ");
            let mut chars = split.next().unwrap().chars();

            let key = (chars.next().unwrap(), chars.next().unwrap());
            rules.insert(key, split.next().unwrap().chars().next().unwrap());
        }

        Polymerization { template, rules }
    }

    /// Performs the given number of insertions and returns the quantity of the most common element
    /// minus the quantity of the least common element.
    fn score(&self, times: usize) -> usize {
        let mut count: HashMap<(char, char), usize> = HashMap::new();

        // Load the template into count.
        for (a, b) in self.template.iter().zip(self.template.iter().skip(1)) {
            *count.entry((*a, *b)).or_default() += 1;
        }

        // Build the polymer.
        for _ in 0..times {
            let mut new_count = HashMap::new();

            for ((a, b), num) in count {
                let letter = self.rules[&(a, b)];
                *new_count.entry((a, letter)).or_default() += num;
                *new_count.entry((letter, b)).or_default() += num;
            }

            count = new_count;
        }

        // Count individual letters.  Since pairs overlap, all letters except the
        // first and last are double-counted.
        let mut letter_count: HashMap<char, usize> = HashMap::new();
        *letter_count.entry(self.template[0]).or_default() += 1;
        *letter_count.entry(self.template[self.template.len() - 1]).or_default() += 1;
        for ((a, b), num) in count {
            *letter_count.entry(a).or_default() += num;
            *letter_count.entry(b).or_default() += num;
        }

        letter_count.values_mut().for_each(|num| *num /= 2);

        // Score is the most common element - least common element.
        letter_count.values().max().unwrap() - letter_count.values().min().unwrap()
    }
}

#[test]
fn test_sample() {
    let poly = Polymerization::load("input/day14_sample.txt");

    assert_eq!(1588, poly.score(10));
    assert_eq!(2188189693529, poly.score(40))
}
