use std::{collections::HashSet, ops::AddAssign};

use aoc_utils::{print_day_header, read_input_file};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// Regex for parsing input string
    static ref RE: Regex = Regex::new(r"([RDUL])(\d+)").unwrap();
}

/// Represents a movement with direction and distance
struct Movement {
    direction: u8,
    distance: usize,
}

impl Movement {
    /// Creates a new movement.
    fn new(direction: &str, distance: &str) -> Self {
        Movement {
            direction: direction.as_bytes()[0],
            distance: distance.parse().unwrap(),
        }
    }
}

/// Represents a point with x and y coordinates
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point(i32, i32);

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

trait ToPoints {
    /// Converts to a hash shet of points
    fn to_points(&self) -> HashSet<Point>;
}

impl ToPoints for str {
    fn to_points(&self) -> HashSet<Point> {
        let mut current_position = Point(0i32, 0i32);
        let mut points: HashSet<Point> = HashSet::new();
        RE.captures_iter(self)
            .map(|m| Movement::new(&m[1], &m[2]))
            .for_each(|m| {
                for _ in 0..m.distance {
                    current_position += Point(
                        match m.direction {
                            b'L' => -1,
                            b'R' => 1,
                            _ => 0,
                        },
                        match m.direction {
                            b'D' => -1,
                            b'U' => 1,
                            _ => 0,
                        },
                    );
                    points.insert(current_position);
                }
            });

        points
    }
}

/// Finds the distance to the nearest intersection
fn find_distance_to_nearest_intersection(first_line: &str, second_line: &str) -> i32 {
    first_line
        .to_points()
        .intersection(&second_line.to_points())
        .map(|i| i.0.abs() + i.1.abs())
        .min()
        .unwrap()
}

fn main() {
    print_day_header(3);

    let input_text = read_input_file(3);
    let new_line_index = input_text.find('\n').unwrap();

    println!(
        "  Result Star 1: {:?}",
        find_distance_to_nearest_intersection(
            &input_text[..new_line_index],
            &input_text[(new_line_index + 1)..]
        )
    );
}

/// Tests for star 1
#[cfg(test)]
mod tests_star1 {
    use super::*;

    #[test]
    fn to_points_horizontal() {
        let expected: HashSet<Point> = [Point(1, 0), Point(2, 0), Point(3, 0)]
            .into_iter()
            .collect();
        assert_eq!(expected, "R3".to_points());
    }

    #[test]
    fn to_points_vertical() {
        let expected: HashSet<Point> = [Point(0, 1), Point(0, 2), Point(0, 3)]
            .into_iter()
            .collect();
        assert_eq!(expected, "U3".to_points());
    }

    #[test]
    fn to_points_combined() {
        let expected: HashSet<Point> = [Point(0, 1), Point(0, 2), Point(1, 2), Point(2, 2)]
            .into_iter()
            .collect();
        assert_eq!(expected, "U2,R2".to_points());
    }

    #[test]
    fn test_1() {
        assert_eq!(
            6,
            find_distance_to_nearest_intersection("R8,U5,L5,D3", "U7,R6,D4,L4")
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            159,
            find_distance_to_nearest_intersection(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            )
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            135,
            find_distance_to_nearest_intersection(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )
        );
    }
}
