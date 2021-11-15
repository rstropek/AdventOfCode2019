use std::collections::HashMap;

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

impl Point {
    /// Moves the current point one field into given direction
    fn move_point(&mut self, direction: u8) {
        self.0 += match direction {
            b'L' => -1,
            b'R' => 1,
            _ => 0,
        };
        self.1 += match direction {
            b'D' => -1,
            b'U' => 1,
            _ => 0,
        };
    }
}

trait PointCalculator {
    /// Parses content into movements and calculates all visited points from them
    fn to_points(&self) -> HashMap<Point, usize>;

    // Parses content into points and executes given closure on each of them
    fn for_each_point(&self, op: impl FnMut(Point, usize));

    // Parses content into points and finds nearest intersection. The distance is calculated using a provided closure.
    fn find_intersection(&self, other: &str, distance_fn: fn(Point, usize, &HashMap<Point, usize>) -> usize) -> usize;

    // Finds nearest intersection to origin (Manhattan distance; star 1)
    fn find_nearest_intersection(&self, other: &str) -> usize;

    // Finds intersection with shortest path (star 2)
    fn find_shortest_intersection(&self, other: &str) -> usize;

}

impl PointCalculator for str {
    fn to_points(&self) -> HashMap<Point, usize> {
        // Target for gathering all points
        let mut points: HashMap<Point, usize> = HashMap::new();

        // Iterate over all points
        self.for_each_point(|current_position, distance_from_origin| {
            points
                .entry(current_position)
                .or_insert(distance_from_origin);
        });

        points
    }

    fn find_nearest_intersection(&self, other: &str) -> usize {
        self.find_intersection(other, |current_position, _, _| {
            current_position.0.abs() as usize + current_position.1.abs() as usize
        })
    }

    fn find_shortest_intersection(&self, other: &str) -> usize {
        self.find_intersection(other, |current_position, distance_from_origin, other_points| {
            other_points.get(&current_position).unwrap() + distance_from_origin
        })
    }

    fn find_intersection(&self, other: &str, distance_fn: fn(Point, usize, &HashMap<Point, usize>) -> usize) -> usize {
        let mut shortest_distance = usize::MAX;
        let other_points = other.to_points();
        self.for_each_point(|current_position, distance_from_origin| {
            if other_points.contains_key(&current_position) {
                let distance = distance_fn(current_position, distance_from_origin, &other_points);
                if distance < shortest_distance {
                    shortest_distance = distance;
                }
            }
        });

        shortest_distance
    }

    fn for_each_point(&self, mut op: impl FnMut(Point, usize)) {
        let mut current_position = Point(0i32, 0i32);
        let mut distance_from_origin = 0usize;
        RE.captures_iter(self)
            .map(|m| Movement::new(&m[1], &m[2]))
            .for_each(|m| {
                for _ in 0..m.distance {
                    distance_from_origin += 1;
                    current_position.move_point(m.direction);
                    op(current_position, distance_from_origin);
                }
            });
    }
}

fn main() {
    print_day_header(3);

    let input_text = read_input_file(3);
    let new_line_index = input_text.find('\n').unwrap();

    println!(
        "  Result Star 1: {:?}",
        &input_text[..new_line_index]
            .find_nearest_intersection(&input_text[(new_line_index + 1)..])
    );

    println!(
        "  Result Star 2: {:?}",
        &input_text[..new_line_index]
            .find_shortest_intersection(&input_text[(new_line_index + 1)..])
    );
}

/// Tests for star 1
#[cfg(test)]
mod tests_star1 {
    use super::*;

    #[test]
    fn to_points_horizontal() {
        let expected: HashMap<Point, usize> =
            [(Point(1, 0), 1), (Point(2, 0), 2), (Point(3, 0), 3)]
                .into_iter()
                .collect();
        assert_eq!(expected, "R3".to_points());
    }

    #[test]
    fn to_points_vertical() {
        let expected: HashMap<Point, usize> =
            [(Point(0, 1), 1), (Point(0, 2), 2), (Point(0, 3), 3)]
                .into_iter()
                .collect();
        assert_eq!(expected, "U3".to_points());
    }

    #[test]
    fn to_points_combined() {
        let expected: HashMap<Point, usize> = [
            (Point(0, 1), 1),
            (Point(0, 2), 2),
            (Point(1, 2), 3),
            (Point(2, 2), 4),
        ]
        .into_iter()
        .collect();
        assert_eq!(expected, "U2,R2".to_points());
    }

    #[test]
    fn test_1() {
        assert_eq!(6, "R8,U5,L5,D3".find_nearest_intersection("U7,R6,D4,L4"));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            159,
            "R75,D30,R83,U83,L12,D49,R71,U7,L72"
                .find_nearest_intersection("U62,R66,U55,R34,D71,R55,D58,R83")
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            135,
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"
                .find_nearest_intersection("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
        );
    }
}

/// Tests for star 2
#[cfg(test)]
mod tests_star2 {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(30, "R8,U5,L5,D3".find_shortest_intersection("U7,R6,D4,L4"));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            610,
            "R75,D30,R83,U83,L12,D49,R71,U7,L72"
                .find_shortest_intersection("U62,R66,U55,R34,D71,R55,D58,R83")
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            410,
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"
                .find_shortest_intersection("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
        );
    }
}
