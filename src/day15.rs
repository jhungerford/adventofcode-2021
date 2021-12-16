use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn solution() {
    let map = Map::load("input/day15.txt");

    println!("Part 1: {}", map.total_risk());
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

    fn risk(&self, pos: &Position) -> i32 {
        self.risk[pos.row][pos.col]
    }

    /// Finds a path from the top left corner to the bottom right with
    /// the lowest risk.
    fn total_risk(&self) -> i32 {
        // A* search to find the path with the lowest risk.
        let start = ToExplore::start(&self);
        let end = Position::end(&self);

        // Nodes we still need to explore.
        let mut explore = BinaryHeap::new();
        explore.push(start.clone());

        // Map of position to the lowest total risk to get to that position.
        let mut pos_risk = HashMap::new();
        pos_risk.insert(start.pos, start.cost);

        // Explore nodes with the lowest heuristic cost until we reach the end.
        while let Some(node) = explore.pop() {
            if node.pos == end {
                return node.risk;
            }

            for neighbor in node.pos.neighbors(&self) {
                let neighbor_risk = node.risk + self.risk(&neighbor);
                let current_risk = *pos_risk.get(&neighbor).unwrap_or(&i32::MAX);

                if neighbor_risk < current_risk {
                    pos_risk.insert(neighbor, neighbor_risk);
                    explore.push(node.to(neighbor, &self));
                }
            }
        }

        panic!("No path found.")
    }
}

/// `ToExplore` captures an unexplored node during an A* search.  A* uses a heuristic score
/// to find the shortest path between two nodes.  Risk is the actual cost to travel from
/// start to the given position, and cost is risk + a guess of the cost to travel from
/// pos to end.
#[derive(Debug, Eq, PartialEq, Clone)]
struct ToExplore {
    /// Position on the map.
    pos: Position,
    /// Total actual risk from start to pos.
    risk: i32,
    /// Best guess at the risk from start to end traveling through this position.
    cost: i32,
}

impl ToExplore {
    /// Returns a new `ToExplore` at the starting square 0,0.  The starting square is never
    /// entered, so it's risk doesn't count.
    fn start(map: &Map) -> Self {
        let pos = Position::new(0, 0);

        ToExplore {
            pos,
            risk: 0,
            cost: Self::heuristic_cost(&pos, map),
        }
    }

    /// Returns a new `ToExplore` that travels from this position to the given neighbor.
    fn to(&self, pos: Position, map: &Map) -> Self {
        let risk = self.risk + map.risk(&pos);
        let cost = risk + Self::heuristic_cost(&pos, map);

        ToExplore { pos, risk, cost }
    }

    /// Returns the heuristic cost from the given position to the end.
    /// All squares have 1 cost.
    fn heuristic_cost(pos: &Position, map: &Map) -> i32 {
        (map.risk.len() - pos.row - 1) as i32 + (map.risk[0].len() - pos.col - 1) as i32
    }
}

impl Ord for ToExplore {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for ToExplore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Position represents a row and column on the map.
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    /// Creates a new Position at the given row and col.
    fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

    /// Returns valid neighbors around the given position on the map.  Neighbors are up, down,
    /// left, and right - diagonals are not valid.
    fn neighbors<'a>(&'a self, map: &'a Map) -> impl Iterator<Item=Position> + 'a {
        [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter()
            .filter(|&(add_row, add_col)|
                (add_row >= 0 || self.row > 0)
                    && (add_col >= 0 || self.col > 0)
                    && (add_row <= 0 || self.row < map.risk.len() - 1)
                    && (add_col <= 0 || self.col < map.risk[0].len() - 1)
            )
            .map(|(add_row, add_col)| Position::new(
                (self.row as i32 + add_row) as usize,
                (self.col as i32 + add_col) as usize,
            ))
    }

    /// Returns the end position on the map.
    fn end(map: &Map) -> Self {
        Position::new(map.risk.len() - 1, map.risk[0].len() - 1)
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[test]
fn test_sample() {
    let map = Map::load("input/day15_sample.txt");
    assert_eq!(40, map.total_risk());
}
