use day_12::shared;
use lib::point::{Point, DOWN, LEFT, RIGHT, UP};
use std::{collections::HashSet, time::Instant};

fn main() {
    let instant = Instant::now();
    let result = run_part(include_str!("../input.txt"));
    let duration = instant.elapsed();
    println!("Result for day-12 part-2 == {}", result);
    println!("Took about {:?}", duration);
}

// Number of sides = number of vertices. And there is 8 ways for a 'point' to be a vertex.
fn number_of_vertices(points: &HashSet<Point>) -> u32 {
    let mut vertices = 0;

    for &point in points {
        let up = point + UP;
        let down = point + DOWN;
        let left = point + LEFT;
        let right = point + RIGHT;
        let up_left = up + LEFT;
        let up_right = up + RIGHT;
        let down_left = down + LEFT;
        let down_right = down + RIGHT;

        if !points.contains(&up) && !points.contains(&left) {
            vertices += 1;
        }

        if !points.contains(&up) && !points.contains(&right) {
            vertices += 1;
        }

        if !points.contains(&down) && !points.contains(&left) {
            vertices += 1;
        }

        if !points.contains(&down) && !points.contains(&right) {
            vertices += 1;
        }

        if points.contains(&down) && points.contains(&right) && !points.contains(&down_right) {
            vertices += 1;
        }

        if points.contains(&down) && points.contains(&left) && !points.contains(&down_left) {
            vertices += 1;
        }

        if points.contains(&up) && points.contains(&right) && !points.contains(&up_right) {
            vertices += 1;
        }

        if points.contains(&up) && points.contains(&left) && !points.contains(&up_left) {
            vertices += 1;
        }
    }

    vertices
}

fn run_part(input: &str) -> u32 {
    shared::run_part(input, number_of_vertices)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let result = run_part(include_str!("../test.txt"));

        assert_eq!(result, 1206)
    }
}
