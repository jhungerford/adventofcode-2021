use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn solution() {
    let cave_system = CaveSystem::load("input/day12.txt");

    println!("Part 1: {}", cave_system.paths(false));
    println!("Part 2: {}", cave_system.paths(true));
}

#[derive(Debug)]
struct CaveSystem {
    caves: HashMap<String, HashSet<String>>
}

impl CaveSystem {
    /// Loads a cave system from the given file, which describes connected
    /// caves like 'start-A'.
    fn load(filename: &str) -> CaveSystem {
        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);

        let mut caves: HashMap<String, HashSet<String>> = HashMap::new();

        for line in f.lines() {
            let cave_connection = line.unwrap()
                .split("-")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            let cave_a = cave_connection[0].clone();
            let cave_b = cave_connection[1].clone();

            caves.entry(cave_a.clone()).or_default().insert(cave_b.clone());
            caves.entry(cave_b.clone()).or_default().insert(cave_a.clone());
        }

        CaveSystem { caves }
    }

    /// Returns the number of paths through this cave system.
    /// Paths start at 'start', end at 'end', and can travel through capitalized
    /// caves more than once.  If small_twice is true, the sub can travel through a single
    /// lowercase cave (other than start or end) twice.
    fn paths(&self, small_twice: bool) -> usize {
        #[derive(Debug)]
        struct Explore {
            visited: HashSet<String>,
            at: String,
            small_twice: bool
        }

        impl Explore {
            /// Returns a new Explore at the 'start' cave.
            fn start() -> Self {
                Explore {
                    visited: vec!["start".to_string()].into_iter().collect(),
                    at: "start".to_string(),
                    small_twice: false,
                }
            }

            /// Returns whether the sub can visit the given cave.  Caves with capital names
            /// can be visited more than once.
            fn can_visit(&self, cave: &str, small_twice: bool) -> bool {
                // Can visit big caves or caves we've never visited before any number of times.
                let is_big = cave.chars().next().unwrap().is_uppercase();
                if is_big || !self.visited.contains(cave) {
                    return true;
                }

                // Can visit small caves twice if we're allowed.
                if small_twice && !self.small_twice && "start" != cave && "end" != cave {
                    return true;
                }

                false
            }

            /// Returns a new Explore with the sub at the given cave.
            fn visit(&self, cave: &str) -> Self {
                let is_small_twice = cave.chars().next().unwrap().is_lowercase()
                    && "start" != cave
                    && "end" != cave
                    && self.visited.contains(cave);

                let mut visited: HashSet<String> = self.visited.iter().cloned().collect();
                visited.insert(cave.to_string());

                Explore {
                    visited,
                    at: cave.to_string(),
                    small_twice: self.small_twice || is_small_twice,
                }
            }
        }

        let mut to_explore = Vec::new();

        // Always start at start.
        to_explore.push(Explore::start());

        // Keep exploring until we've found all of the paths.
        let mut paths = 0;
        while let Some(explore) = to_explore.pop() {
            if explore.at == "end" {
                paths += 1;
            } else {
                for neighbor in &self.caves[&explore.at] {
                    if explore.can_visit(&neighbor, small_twice) {
                        to_explore.push(explore.visit(&neighbor));
                    }
                }
            }
        }

        paths
    }
}

#[test]
fn paths_samples() {
    assert_eq!(10, CaveSystem::load("input/day12_sample.txt").paths(false));
    assert_eq!(19, CaveSystem::load("input/day12_sample2.txt").paths(false));
    assert_eq!(226, CaveSystem::load("input/day12_sample3.txt").paths(false));

    assert_eq!(36, CaveSystem::load("input/day12_sample.txt").paths(true));
    assert_eq!(103, CaveSystem::load("input/day12_sample2.txt").paths(true));
    assert_eq!(3509, CaveSystem::load("input/day12_sample3.txt").paths(true));
}
