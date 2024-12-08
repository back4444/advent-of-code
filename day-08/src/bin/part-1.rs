use day_08::shared;
use std::collections::HashSet;

fn main() {
    let result = run_part(include_str!("../input.txt"));
    println!("Result for day-08 part-1 == {}", result)
}

fn run_part(input: &str) -> u32 {
    let (antennas, n) = shared::parse_antennas(input);

    let mut antinode_positions = HashSet::new();

    for (i, a) in antennas.iter().enumerate() {
        for b in antennas.iter().skip(i + 1) {
            if a.freq != b.freq {
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
