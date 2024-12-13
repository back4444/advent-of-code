use std::time::Instant;

use day_13::shared;

fn main() {
    let instant = Instant::now();
    let result = run_part(include_str!("../input.txt"));
    let duration = instant.elapsed();
    println!("Result for day-13 part-2 == {}", result);
    println!("Took about {:?}", duration);
}

fn run_part(input: &str) -> u64 {
    input
        .split("\n\n")
        .flat_map(|block| {
            let points: Vec<(u64, u64)> = block
                .lines()
                .map(|line| {
                    let mut parts = line.split(':').last().unwrap().split(',');

                    let x = parts.next().unwrap().split(['+', '=']).last().unwrap();
                    let y = parts.next().unwrap().split(['+', '=']).last().unwrap();

                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect();

            shared::solve_system(
                points[0],
                points[1],
                (points[2].0 + 10000000000000, points[2].1 + 10000000000000),
            )
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let result = run_part(&include_str!("../test.txt").replace("\r", ""));

        assert_eq!(result, 8753186089080)
    }
}
