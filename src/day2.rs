use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::str::FromStr;

#[allow(dead_code)]
pub fn solution() {
    let directions = load("input/day2.txt");

    println!("Part 1: {}", distance(&directions));
    println!("Part 2: {}", aim_distance(&directions));
}

fn load(filename: &str) -> Vec<Direction> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    f.lines()
        .flat_map(|line| line.unwrap().parse::<Direction>())
        .collect()
}

/// Follows the given directions and returns the horizontal position multiplied by the final depth.
fn distance(directions: &Vec<Direction>) -> i32 {
    let end_position= directions.iter().fold(Position::default(), |pos, dir| pos + dir);

    end_position.depth * end_position.distance
}

/// Follows the given directions and returns horizontal position * depth.  Up and down directions
/// aim the submarine instead of moving it up and down.
fn aim_distance(directions: &Vec<Direction>) -> i32 {
    let end_position= directions.iter().fold(AimPosition::default(), |pos, dir| pos + dir);

    end_position.depth * end_position.distance
}

#[derive(Debug, Eq, PartialEq)]
struct ParseErr {}

/// Direction describes where and how far the submarine should move.
#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Direction {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // String looks like 'forward 5'
        let mut parts = s.split_whitespace();

        match (parts.next(), parts.next()) {
            (Some("forward"), Some(amount)) => Ok(Direction::Forward(amount.parse().unwrap())),
            (Some("down"), Some(amount)) => Ok(Direction::Down(amount.parse().unwrap())),
            (Some("up"), Some(amount)) => Ok(Direction::Up(amount.parse().unwrap())),
            _ => Err(ParseErr {}),
        }
    }
}

/// Position captures the submarine's depth and forward distance.  Up and Down directions
/// move the sub up and down.
#[derive(Debug, Eq, PartialEq)]
struct Position {
    depth: i32,
    distance: i32,
}

impl Position {
    fn new(depth: i32, distance: i32) -> Self {
        Position { depth, distance }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Add<&Direction> for Position {
    type Output = Position;

    fn add(self, dir: &Direction) -> Self::Output {
        match dir {
            Direction::Forward(amount) => Position {
                depth: self.depth,
                distance: self.distance + amount,
            },
            Direction::Down(amount) => Position {
                depth: self.depth + amount,
                distance: self.distance,
            },
            Direction::Up(amount) => Position {
                depth: self.depth - amount,
                distance: self.distance,
            },
        }
    }
}

/// AimPosition captures the sub's depth, forward distance, and aim angle.  Up and Down directions
/// adjust the sub's aim, and Forward adjusts it's forward and horizontal positions based on the aim.
#[derive(Debug, Eq, PartialEq)]
struct AimPosition {
    depth: i32,
    distance: i32,
    aim: i32,
}

impl AimPosition {
    fn new(depth: i32, distance: i32, aim: i32) -> Self {
        AimPosition { depth, distance, aim }
    }
}

impl Default for AimPosition {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl Add<&Direction> for AimPosition {
    type Output = AimPosition;

    fn add(self, dir: &Direction) -> Self::Output {
        match dir {
            Direction::Forward(amount) => AimPosition {
                depth: self.depth + self.aim * amount,
                distance: self.distance + amount,
                aim: self.aim,
            },
            Direction::Down(amount) => AimPosition {
                depth: self.depth,
                distance: self.distance,
                aim: self.aim + amount,
            },
            Direction::Up(amount) => AimPosition {
                depth: self.depth,
                distance: self.distance,
                aim: self.aim - amount,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_directions() {
        assert_eq!(Ok(Direction::Forward(3)), "forward 3".parse());
        assert_eq!(Ok(Direction::Down(5)), "down 5".parse());
        assert_eq!(Ok(Direction::Up(10)), "up 10".parse());
        assert_eq!(Err(ParseErr {}), "not a direction".parse::<Direction>());
    }

    #[test]
    fn add_direction() {
        assert_eq!(Position::new(0, 3), Position::default() + &Direction::Forward(3));
        assert_eq!(Position::new(5, 0), Position::default() + &Direction::Down(5));
        assert_eq!(Position::new(-10, 0), Position::default() + &Direction::Up(10));
    }

    #[test]
    fn test_distance() {
        let directions = vec![
            Direction::Forward(5),
            Direction::Down(5),
            Direction::Forward(8),
            Direction::Up(3),
            Direction::Down(8),
            Direction::Forward(2),
        ];

        assert_eq!(150, distance(&directions))
    }

    #[test]
    fn add_aim_direction() {
        assert_eq!(AimPosition::new(0, 3, 0), AimPosition::default() + &Direction::Forward(3));
        assert_eq!(AimPosition::new(15, 3, 5), AimPosition::new(0, 0, 5) + &Direction::Forward(3));
        assert_eq!(AimPosition::new(0, 0, 5), AimPosition::default() + &Direction::Down(5));
        assert_eq!(AimPosition::new(0, 0, -10), AimPosition::default() + &Direction::Up(10));
    }

    #[test]
    fn test_aim_distance() {
        let directions = vec![
            Direction::Forward(5),
            Direction::Down(5),
            Direction::Forward(8),
            Direction::Up(3),
            Direction::Down(8),
            Direction::Forward(2),
        ];

        assert_eq!(900, aim_distance(&directions))
    }
}