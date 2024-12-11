use std::time::Instant;

use day_10::shared;
use lib::point::Point;

fn main() {
    let instant = Instant::now();
    let result = run_part(include_str!("../input.txt"));
    let duration = instant.elapsed();
    println!("Result for day-10 part-1 == {}", result);
    println!("Took about {:?}", duration);
}

fn run_part(input: &str) -> u32 {
    let mut grid = shared::parse_grid(input);
    let mut total = 0;

    for (y, row) in grid.clone().iter().enumerate() {
        for (x, &entry) in row.iter().enumerate() {
            if entry == 0 {
                let mut paths: Vec<Point> =
                    shared::find_paths(&mut grid, Point::new(x as i32, y as i32))
                        .iter()
                        .map(|path| *path.last().unwrap())
                        .collect();

                paths.dedup();
                total += paths.len() as u32
            }
        }
    }

    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let result = run_part(include_str!("../test.txt"));

        assert_eq!(result, 36)
    }
}
