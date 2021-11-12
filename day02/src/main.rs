use aoc_utils::{print_day_header, read_input_file};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// Regex for parsing input string
    static ref RE: Regex = Regex::new(r"\d+").unwrap();
}

/// Get op codes by parsing input string
fn get_op_codes(contents: String) -> Vec<usize> {
    RE.find_iter(&contents)
        .map(|l| l.as_str().parse().unwrap())
        .collect()
}

const HALT: usize = 99;
const ADD: usize = 1;
const MULT: usize = 2;

/// Execute intcode program
fn execute(op_codes: &mut [usize]) {
    let mut ip = 0usize; // Instructure pointer
    loop {
        // Op code 99 stops program
        if op_codes[ip] == HALT {
            break;
        }

        // Get addresses
        let target = op_codes[ip + 3];
        let op1 = op_codes[ip + 1];
        let op2 = op_codes[ip + 2];

        // Interpret op codes
        match op_codes[ip] {
            ADD => op_codes[target] = op_codes[op1] + op_codes[op2],
            MULT => op_codes[target] = op_codes[op1] * op_codes[op2],
            _ => panic!("Invalid op code {}", op_codes[ip]),
        };

        // Forward instruction pointer
        ip += 4;
    }
}

/// Restore state to 1202 program alert and execute program
fn restore_alert_and_execute(op_codes: &[usize]) -> usize {
    let mut op_codes = op_codes.to_owned();
    op_codes[1] = 12;
    op_codes[2] = 2;
    execute(&mut op_codes);
    op_codes[0]
}

/// Find noun and verb
fn find_noun_verb(op_codes: &[usize]) -> usize {
    const RESULT: usize = 19690720;

    // Try all nouns and verbs between 0 and 99
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut op_codes = op_codes.to_owned();
            op_codes[1] = noun;
            op_codes[2] = verb;
            execute(&mut op_codes);
            if op_codes[0] == RESULT {
                return 100 * noun + verb;
            }
        }
    }

    panic!("No solution found");
}

fn main() {
    print_day_header(2);

    let op_codes = get_op_codes(read_input_file(2));
    println!("  Result Star 1: {:?}", restore_alert_and_execute(&op_codes));
    println!("  Result Star 2: {:?}", find_noun_verb(&op_codes));
}

/// Tests for star 1
#[cfg(test)]
mod tests_star1 {
    use super::*;

    #[test]
    fn test_1() {
        let mut op_codes: Vec<usize> = get_op_codes(String::from("1,9,10,3,2,3,11,0,99,30,40,50"));
        execute(&mut op_codes);
        assert_eq!(3500, op_codes[0]);
    }

    #[test]
    fn test_2() {
        let mut op_codes: Vec<usize> = get_op_codes(String::from("1,0,0,0,99"));
        execute(&mut op_codes);
        assert_eq!(2, op_codes[0]);
    }

    #[test]
    fn test_3() {
        let mut op_codes: Vec<usize> = get_op_codes(String::from("2,3,0,3,99"));
        execute(&mut op_codes);
        assert_eq!(6, op_codes[3]);
    }

    #[test]
    fn test_4() {
        let mut op_codes: Vec<usize> = get_op_codes(String::from("2,4,4,5,99,0"));
        execute(&mut op_codes);
        assert_eq!(9801, op_codes[5]);
    }

    #[test]
    fn test_5() {
        let mut op_codes: Vec<usize> = get_op_codes(String::from("1,1,1,4,99,5,6,0,99"));
        execute(&mut op_codes);
        assert_eq!(30, op_codes[0]);
    }
}
