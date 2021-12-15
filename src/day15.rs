use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn solution() {
    let map = Map::load("input/day9.txt");

    println!("Part 1: {}", map.risk());
}

struct Map {
    risk: Vec<Vec<i32>>
}

impl Map {
    /// Loads a HeightMap from the given file.
    fn load(filename: &str) -> Self {
        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);

        let risk = f.lines()
            .map(|line| line.unwrap().chars().map(|c| c as i32 - '0' as i32).collect())
            .collect();

        Map { risk }
    }

    /// Finds a path from the top left corner to the bottom right with
    /// the lowest risk.
    fn risk(&self) -> i32 {
        todo!()
    }
}

#[test]
fn test_sample() {
    let map = Map::load("input/day15_sample.txt");
    assert_eq!(40, map.risk());
}
