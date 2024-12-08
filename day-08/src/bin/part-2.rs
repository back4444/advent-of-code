use lib::point::Point;
use std::collections::HashSet;

fn main() {
    let result = run_part(include_str!("../input.txt"));
    println!("Result for day-08 part-2 == {}", result)
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

            let mut a_antinode_pos = a.pos;
            let mut b_antinode_pos = b.pos;

            while a_antinode_pos.is_on_grid(n) {
                antinode_positions.insert(a_antinode_pos);
                a_antinode_pos += diff;
            }

            while b_antinode_pos.is_on_grid(n) {
                antinode_positions.insert(b_antinode_pos);
                b_antinode_pos -= diff;
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

        assert_eq!(result, 34)
    }
}
