pub mod shared {
    use lib::point::{Point, UP};
    use std::collections::HashSet;

    pub fn parse_grid_and_guard_pos(input: &str) -> (Vec<Vec<char>>, Point) {
        let mut grid = vec![];
        let mut guard_pos = Point::new(0, 0);

        for (i, line) in input.lines().enumerate() {
            let mut row = vec![];

            for (j, ch) in line.char_indices() {
                row.push(ch);

                if ch == '^' {
                    guard_pos = Point::new(j as i32, i as i32);
                }
            }

            grid.push(row);
        }

        (grid, guard_pos)
    }

    pub fn get_char(grid: &[Vec<char>], pos: Point) -> Option<&char> {
        grid.get(pos.y as usize)
            .and_then(|row| row.get(pos.x as usize))
    }

    pub fn calculate_visited(grid: &[Vec<char>], start_pos: Point) -> HashSet<Point> {
        let mut pos = start_pos;
        let mut visited: HashSet<Point> = HashSet::new();
        let mut vector = UP;

        loop {
            let next_pos = pos + vector;

            match get_char(grid, next_pos) {
                Some('#') => vector = vector.clockwise(),
                Some(_) => {
                    pos = next_pos;
                    visited.insert(pos);
                }
                _ => break,
            }
        }

        visited
    }
}
