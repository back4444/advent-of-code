// Extremely slow but it does work!

use day_06::shared;
use lib::point::{Point, UP};
use std::collections::HashSet;

fn main() {
    let result = run_part(include_str!("../input.txt"));
    println!("Result for day-06 part-2 == {}", result)
}

fn run_part(input: &str) -> u32 {
    let (mut grid, start_pos) = shared::parse_grid_and_guard_pos(input);

    let mut detected_loops = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '.' {
                grid[i][j] = '#';

                let mut guard_pos = start_pos;
                let mut vector = UP;

                let mut visited: HashSet<(Point, Point)> = HashSet::new();

                loop {
                    let next_pos = guard_pos + vector;

                    match shared::get_char(&grid, next_pos) {
                        Some('#') => vector = vector.clockwise(),
                        Some(_) => {
                            guard_pos = next_pos;

                            if !visited.insert((guard_pos, vector)) {
                                detected_loops += 1;
                                break;
                            }
                        }
                        _ => break,
                    }
                }

                grid[i][j] = '.';
            }
        }
    }

    detected_loops
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let result = run_part(include_str!("../test.txt"));

        assert_eq!(result, 6)
    }
}
