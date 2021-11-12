use lazy_static::lazy_static;
use regex::Regex;
use aoc_utils::{print_day_header, read_input_file};

/// Calculate fuel for a given mass (logic for star 1)
fn get_fuel_star1(mass: i32) -> i32 {
    ((mass as f64) / 3.0).floor() as i32 - 2
}

/// Calculate fuel for a given mass (logic for star 2)
fn get_fuel_star2(mass: i32) -> i32 {
    let mut total_fuel = 0;
    let mut mass = mass;

    loop {
        let fuel = get_fuel_star1(mass);
        if fuel > 0 {
            total_fuel += fuel;
            mass = fuel;
        } else {
            break;
        }
    }

    total_fuel
}

/// Calculates fuel for given masses with given calculator function
fn get_total_fuel(masses: &[i32], mass_calculator: fn (i32) -> i32) -> i32 {
    masses.iter().map(|m| mass_calculator(*m)).sum()
}

lazy_static! {
    /// Regex for parsing input string
    static ref RE: Regex = Regex::new(r"\d+").unwrap();
}

/// Get mass values by parsing input string
fn get_masses(contents: String) -> Vec<i32> {
    RE.find_iter(&contents)
        .map(|l| l.as_str().parse().unwrap())
        .collect()
}

fn main() {
    print_day_header(1);

    // Get masses. This is done once because input is the same for both stars.
    let masses = get_masses(read_input_file(1));

    // Star 1
    let total_fuel = get_total_fuel(&masses, get_fuel_star1);
    println!("  Result Star 1: {:?}", total_fuel);

    // Star 2
    let total_fuel = get_total_fuel(&masses, get_fuel_star2);
    println!("  Result Star 2: {:?}", total_fuel);
}

/// Tests for star 1
#[cfg(test)]
mod tests_star1 {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(get_fuel_star1(12), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(get_fuel_star1(14), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(get_fuel_star1(1969), 654);
    }

    #[test]
    fn test_4() {
        assert_eq!(get_fuel_star1(100756), 33583);
    }
}

/// Tests for star 2
#[cfg(test)]
mod tests_star2 {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(get_fuel_star2(14), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(get_fuel_star2(1969), 966);
    }

    #[test]
    fn test_3() {
        assert_eq!(get_fuel_star2(100756), 50346);
    }
}
