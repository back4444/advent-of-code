use day_14::shared;
use lib::point::{Point, RIGHT};
use std::{collections::HashSet, time::Instant};

fn main() {
    let instant = Instant::now();
    run_part(include_str!("../input.txt"));
    let duration = instant.elapsed();
    println!("Took about {:?}", duration);
}

fn run_part(input: &str) {
    let positions = shared::parse_positions(input);

    for n in 0..10_000 {
        let points: HashSet<Point> = positions
            .iter()
            .map(|(point, vector)| {
                Point::new(
                    (point.x + n * vector.x).rem_euclid(shared::GRID_X),
                    (point.y + n * vector.y).rem_euclid(shared::GRID_Y),
                )
            })
            .collect();

        // Strategy: search for any set of points up to 10,000 seconds,
        // looking for 10 points of in a row...

        let mut row = false;

        for point in &points {
            if (0..10).all(|i| points.contains(&(*point + RIGHT * i))) {
                row = true;
            }
        }

        if row {
            println!("{n}");

            for y in 0..shared::GRID_Y {
                for x in 0..shared::GRID_Y {
                    if points.contains(&Point::new(x, y)) {
                        print!("#")
                    } else {
                        print!(".")
                    }
                }
                println!()
            }
        }

        // Easter egg found at n = 8159
    }
}
