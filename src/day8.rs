use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[allow(dead_code)]
pub fn solution() {
    let entries = load("input/day8.txt");

    println!("Part 1: {}", num_unique(&entries));
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

#[derive(Debug)]
enum ParseErr {}

#[derive(Debug, Eq, PartialEq)]
struct Entry {
    signal: Vec<String>,
    output: Vec<String>,
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