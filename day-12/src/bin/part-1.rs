use day_12::shared;
use lib::{point::Point, utils};
use std::{collections::HashSet, time::Instant};

fn main() {
    let instant = Instant::now();
    let result = run_part(include_str!("../input.txt"));
    let duration = instant.elapsed();
    println!("Result for day-12 part-1 == {}", result);
    println!("Took about {:?}", duration);
}

fn perimiter_length(points: &HashSet<Point>) -> u32 {
    let mut perimiter = 0;

    for point in points {
        let mut point_perimiter = 4;

        for unit in utils::DIRECTIONS {
            let neighbour = *point + unit;

            if points.contains(&neighbour) {
                point_perimiter -= 1;
            }
        }

        perimiter += point_perimiter;
    }

    perimiter
}

fn run_part(input: &str) -> u32 {
    shared::run_part(input, perimiter_length)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let result = run_part(include_str!("../test.txt"));

        assert_eq!(result, 1930)
    }
}
