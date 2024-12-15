use day_14::shared;
use lib::point::Point;
use std::time::Instant;

fn main() {
    let instant = Instant::now();
    let result = run_part(include_str!("../input.txt"));
    let duration = instant.elapsed();
    println!("Result for day-14 part-1 == {}", result);
    println!("Took about {:?}", duration);
}

fn run_part(input: &str) -> u32 {
    let positions = shared::parse_positions(input);

    let n = 100;

    let quad_x = (shared::GRID_X - 1) / 2;
    let quad_y = (shared::GRID_Y - 1) / 2;

    let (mut top_left, mut top_right, mut bottom_left, mut bottom_right) = (0, 0, 0, 0);

    for (point, vector) in positions {
        let new_point = Point::new(
            (point.x + n * vector.x).rem_euclid(shared::GRID_X),
            (point.y + n * vector.y).rem_euclid(shared::GRID_Y),
        );

        match (new_point.x < quad_x, new_point.y < quad_y) {
            (true, true) => top_left += 1,
            (false, true) => top_right += 1,
            (true, false) => bottom_left += 1,
            (false, false) => bottom_right += 1,
        }
    }

    top_left * top_right * bottom_left * bottom_right
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let result = run_part(include_str!("../test.txt"));

        assert_eq!(result, 12)
    }
}
