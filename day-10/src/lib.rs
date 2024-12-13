pub mod shared {
    use lib::{point::Point, utils};

    pub fn parse_grid(input: &str) -> Vec<Vec<i32>> {
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect()
            })
            .collect()
    }

    pub fn find_paths(grid: &mut Vec<Vec<i32>>, start_pos: Point) -> Vec<Vec<Point>> {
        let mut paths = Vec::new();
        let mut current_path = Vec::new();

        fn dfs(
            grid: &mut Vec<Vec<i32>>,
            current_pos: Point,
            current_value: i32,
            current_path: &mut Vec<Point>,
            paths: &mut Vec<Vec<Point>>,
        ) {
            current_path.push(current_pos);

            if grid[current_pos] == 9 {
                paths.push(current_path.clone());
                current_path.pop();
                return;
            }

            let temp = grid[current_pos];
            grid[current_pos] = -1;

            for unit in utils::DIRECTIONS {
                let next_pos = current_pos + unit;

                if next_pos.is_on_grid(grid.len()) && grid[next_pos] == current_value + 1 {
                    dfs(grid, next_pos, current_value + 1, current_path, paths);
                }
            }

            grid[current_pos] = temp;
            current_path.pop();
        }

        dfs(grid, start_pos, 0, &mut current_path, &mut paths);
        paths
    }
}
