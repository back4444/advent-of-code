use std::time::Instant;

use day_13::shared;

fn main() {
    let instant = Instant::now();
    let result = run_part(include_str!("../input.txt"));
    let duration = instant.elapsed();
    println!("Result for day-13 part-1 == {}", result);
    println!("Took about {:?}", duration);
}

fn run_part(input: &str) -> u64 {
    input
        .split("\n\n")
        .flat_map(|block| {
            let mut points = block.lines().map(|line| {
                let mut parts = line.split(':').last().unwrap().split(',');

                let x = parts.next().unwrap().split(['+', '=']).last().unwrap();
                let y = parts.next().unwrap().split(['+', '=']).last().unwrap();

                (x.parse().unwrap(), y.parse().unwrap())
            });

            shared::solve_system(
                points.next().unwrap(),
                points.next().unwrap(),
                points.next().unwrap(),
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

        assert_eq!(result, 480)
    }
}
