// Slightly more optimised version of part-2.
// Instead of testing for loops by placing obstacles at
// every location in the grid, we only place obstacles on
// the guards original path.

use day_06::shared;
use lib::point::{Point, UP};
use std::{collections::HashSet, time::Instant};

fn main() {
    let instant = Instant::now();
    let result = run_part(include_str!("../input.txt"));
    let elapsed = instant.elapsed();

    println!(
        "\n\nResult for day-06 part-2 == {}.\nTook about {:?}\n\n",
        result, elapsed
    )
}

fn run_part(input: &str) -> u32 {
    let (mut grid, start_pos) = shared::parse_grid_and_guard_pos(input);

    let mut loop_count = 0;
    let mut visited: HashSet<(Point, Point)> = HashSet::new();

    for pos in shared::calculate_visited(&grid, start_pos) {
        if pos == start_pos {
            continue;
        }

        grid[pos.y as usize][pos.x as usize] = '#';

        let mut guard_pos = start_pos;
        let mut vector = UP;

        loop {
            let next_pos = guard_pos + vector;

            match shared::get_char(&grid, next_pos) {
                Some('#') => vector = vector.clockwise(),
                Some(_) => {
                    guard_pos = next_pos;

                    if !visited.insert((guard_pos, vector)) {
                        loop_count += 1;
                        break;
                    }
                }
                _ => break,
            }
        }

        visited.clear();

        grid[pos.y as usize][pos.x as usize] = '.';
    }

    loop_count
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
