use std::ops::RangeInclusive;
use regex::Regex;

pub fn solution() {
    let target = Target::parse("target area: x=144..178, y=-100..-76");

    println!("Part 1: {}", target.highest_y());
}

struct Target {
    x_range: RangeInclusive<i64>,
    y_range: RangeInclusive<i64>,
}

impl Target {
    /// Parses a target from the given string like 'target area: x=20..30, y=-10..-5'
    fn parse(s: &str) -> Self {
        let re = Regex::new(r"^target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)").unwrap();
        let captures = re.captures(s).unwrap();

        Target {
            x_range: captures[1].parse().unwrap() ..= captures[2].parse().unwrap(),
            y_range: captures[3].parse().unwrap() ..= captures[4].parse().unwrap(),
        }
    }

    /// Returns the highest y position that the probe can reach and still hit the target area.
    fn highest_y(&self) -> i64 {
        // brute force.
        let mut max_y = 0;

        for x in 0..1000 {
            let mut potential_max_y = 0;
            for y in 0..1000 {
                let mut probe = Probe::new(Point::new(x, y));

                loop {
                    probe.step();
                    potential_max_y = potential_max_y.max(probe.pos.y);

                    if probe.is_in_target(self) {
                        max_y = max_y.max(potential_max_y);
                        break;
                    }

                    if probe.is_miss(self) {
                        break;
                    }
                }

            }
        }

        max_y
    }
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Probe {
    pos: Point,
    vel: Point,
}

impl Probe {
    /// Constructs a new Probe with the given initial velocity.
    fn new(vel: Point) -> Self {
        Probe {
            pos: Point::new(0, 0),
            vel,
        }
    }

    /// Advances this probe's position and velocity by one step.
    fn step(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;

        if self.vel.x > 0 {
            self.vel.x -= 1;
        } else if self.vel.x < 0 {
            self.vel.x += 1;
        }

        self.vel.y -= 1;
    }

    /// Returns whether this probe is in the target zone.
    fn is_in_target(&self, target: &Target) -> bool {
        target.x_range.contains(&self.pos.x) && target.y_range.contains(&self.pos.y)
    }

    /// Returns whether this probe will never hit the target.
    fn is_miss(&self, target: &Target) -> bool {
        // y is below the range.
        if self.pos.y < *target.y_range.start() {
            return true;
        }

        // x is too slow to hit the range.
        if self.vel.x == 0 && !target.x_range.contains(&self.pos.x) {
            return true;
        }

        // x is beyond the range.
        if self.pos.x > *target.x_range.end() {
            return true;
        }

        // Probe might still hit.  Could get smarter here about checking y.
        false
    }
}

#[test]
fn highest_y_sample() {
    let target = Target::parse("target area: x=20..30, y=-10..-5");
    assert_eq!(45, target.highest_y());
}
