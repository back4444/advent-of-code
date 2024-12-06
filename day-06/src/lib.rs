pub mod shared {
    use lib::point::Point;

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
}
