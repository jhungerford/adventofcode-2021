use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn solution() {
    let lines = load("input/day10.txt");

    println!("Part 1: {}", total_score(&lines));
}

fn load(filename: &str) -> Vec<Line> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    f.lines()
        .map(|line| Line::new(line.unwrap()))
        .collect()
}

fn total_score(lines: &Vec<Line>) -> i32 {
    lines.iter().map(|line| line.score()).sum()
}

struct Line {
    line: String
}

impl Line {
    fn new(line: String) -> Self {
        Self { line }
    }

    /// Returns the syntax error score if this line is corrupted, or 0 otherwise.   A line has a
    /// syntax error if a chunk opens and closes with mismatched characters.  Closing with
    /// `)` = 3 points, `]` = 57, `}` = 1197, and `>` = 25137.
    fn score(&self) -> i32 {
        let scores = vec![
            (')', 3),
            (']', 57),
            ('}', 1197),
            ('>', 25137),
        ].into_iter().collect::<HashMap<char, i32>>();

        let open_to_close = vec![
            ('(', ')'),
            ('[', ']'),
            ('{', '}'),
            ('<', '>'),
        ].into_iter().collect::<HashMap<char, char>>();

        let mut stack = Vec::new();

        for c in self.line.chars() {
            if let Some(m) = open_to_close.get(&c) {
                stack.push(m)
            } else if let Some(m) = stack.pop() {
                if *m != c {
                    return scores[&c];
                }
            }
        }

        // Line was incomplete, or didn't have any mismatches.
        0
    }
}

#[test]
fn score_sample_file() {
    let lines = load("input/day10_sample.txt");
    assert_eq!(26397, total_score(&lines));
}