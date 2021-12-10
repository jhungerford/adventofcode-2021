use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

#[allow(dead_code)]
pub fn solution() {
    let lines = load("input/day10.txt");

    println!("Part 1: {}", syntax_score(&lines));
    println!("Part 2: {}", autocomplete_score(&lines));
}

fn load(filename: &str) -> Vec<Line> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    f.lines()
        .map(|line| Line::new(line.unwrap()))
        .collect()
}

/// Returns the sum of syntax scores for lines that have mismatched closing characters.
fn syntax_score(lines: &Vec<Line>) -> i32 {
    lines.iter().flat_map(|line| line.syntax_score()).sum()
}

/// Returns the middle autocomplete score for incomplete lines.
fn autocomplete_score(lines: &Vec<Line>) -> i64 {
    let scores = lines.iter()
        .flat_map(|line| line.autocomplete_score())
        .sorted()
        .collect::<Vec<i64>>();

    scores[scores.len() / 2]
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
    fn syntax_score(&self) -> Option<i32> {
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
                    return Some(scores[&c]);
                }
            }
        }

        // Line was incomplete, or didn't have any mismatches.
        None
    }

    fn autocomplete_score(&self) -> Option<i64> {
        let scores = vec![
            (')', 1),
            (']', 2),
            ('}', 3),
            ('>', 4),
        ].into_iter().collect::<HashMap<char, i64>>();

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
                    // Line is corrupt - mismatched open and close.
                    return None;
                }
            }
        }

        // Line is incomplete - calculate the score.
        let mut score = 0;
        while let Some(c) = stack.pop() {
            score = score * 5 + scores[c];
        }

        Some(score)
    }
}

#[test]
fn score_sample_file() {
    let lines = load("input/day10_sample.txt");
    assert_eq!(26397, syntax_score(&lines));
    assert_eq!(288957, autocomplete_score(&lines));
}