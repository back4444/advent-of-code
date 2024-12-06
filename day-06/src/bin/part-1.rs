use std::collections::HashSet;

use day_06::shared;
use lib::point::{Point, UP};

fn main() {
    let result = run_part(include_str!("../input.txt"));
    println!("Result for day-06 part-1 == {}", result)
}

fn run_part(input: &str) -> u32 {
    let (grid, mut guard_pos) = shared::parse_grid_and_guard_pos(input);

    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(guard_pos);

    let mut vector = UP;

    loop {
        let next_pos = guard_pos + vector;

        match shared::get_char(&grid, next_pos) {
            Some('#') => vector = vector.clockwise(),
            Some('.') | Some('^') => {
                guard_pos = next_pos;
                visited.insert(guard_pos);
            }
            _ => break,
        }
    }

    visited.len() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let result = run_part(include_str!("../test.txt"));

        assert_eq!(result, 41)
    }
}
