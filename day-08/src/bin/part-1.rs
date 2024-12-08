use lib::point::Point;
use std::collections::HashSet;

fn main() {
    let result = run_part(include_str!("../input.txt"));
    println!("Result for day-08 part-1 == {}", result)
}

#[derive(Debug, Clone, Copy)]
struct Antenna {
    frequency: char,
    pos: Point,
}

fn run_part(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let n = grid.len();

    let antennas: Vec<Antenna> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &entry)| (entry != '.'))
                .map(move |(x, &entry)| Antenna {
                    frequency: entry,
                    pos: Point::new(x as i32, y as i32),
                })
        })
        .collect();

    let mut antinode_positions = HashSet::new();

    for (i, a) in antennas.iter().enumerate() {
        for b in antennas.iter().skip(i + 1) {
            if a.frequency != b.frequency {
                continue;
            }

            let diff = a.pos - b.pos;

            let a_antinode_pos = a.pos + diff;
            let b_antinode_pos = b.pos - diff;

            if a_antinode_pos.is_on_grid(n) {
                antinode_positions.insert(a_antinode_pos);
            }

            if b_antinode_pos.is_on_grid(n) {
                antinode_positions.insert(b_antinode_pos);
            }
        }
    }

    antinode_positions.len() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let result = run_part(include_str!("../test.txt"));

        assert_eq!(result, 14)
    }
}
