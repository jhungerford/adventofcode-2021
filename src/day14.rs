use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn solution() {
    let mut poly = Polymerization::load("input/day14.txt");

    poly.step(10);
    println!("Part 1: {}", poly.score());
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

    fn step(&mut self, times: usize) {
        for _ in 0..times {
            let mut new_template = Vec::with_capacity(self.template.len() * 2);

            let pairs = self.template.iter()
                .cloned()
                .zip(self.template.iter().cloned().skip(1));

            for pair in pairs {
                new_template.push(pair.0);
                new_template.push(self.rules[&pair]);
            }

            new_template.push(self.template[self.template.len() - 1]);

            self.template = new_template;
        }
    }

    /// Returns the quantity of the most common element minus the quantity
    /// of the least common element.
    fn score(&self) -> usize {
        let mut element_count: HashMap<&char, usize> = HashMap::new();
        for c in &self.template {
            *element_count.entry(c).or_default() += 1;
        }

        element_count.values().max().unwrap() - element_count.values().min().unwrap()
    }
}

#[test]
fn test_sample() {
    let mut poly = Polymerization::load("input/day14_sample.txt");
    poly.step(10);

    assert_eq!(1588, poly.score());
}
