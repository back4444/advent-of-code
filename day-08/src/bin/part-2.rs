use day_08::shared;
use std::collections::HashSet;

fn main() {
    let result = run_part(include_str!("../input.txt"));
    println!("Result for day-08 part-2 == {}", result)
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
