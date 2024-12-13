pub mod shared {
    use lib::{point::Point, utils};
    use std::collections::{HashSet, VecDeque};

    pub fn run_part<F>(input: &str, function: F) -> u32
    where
        F: Fn(&HashSet<Point>) -> u32,
    {
        let grid = lib::utils::char_grid(input);
        let n = grid.len();

        let mut total = 0;
        let mut seen: HashSet<Point> = HashSet::new();

        for y in 0..n {
            for x in 0..n {
                let point = Point::new(x as i32, y as i32);
                let plant = grid[point];

                if seen.contains(&point) {
                    continue;
                }

                let mut seen_this_iteration: HashSet<Point> = HashSet::new();
                let mut queue = VecDeque::new();

                seen_this_iteration.insert(point);
                queue.push_back(point);

                while !queue.is_empty() {
                    let current_point = queue.pop_front().unwrap();

                    for unit in utils::DIRECTIONS {
                        let new_point = current_point + unit;

                        if new_point.is_on_grid(n)
                            && grid[new_point] == plant
                            && !seen_this_iteration.contains(&new_point)
                        {
                            queue.push_back(new_point);
                            seen_this_iteration.insert(new_point);
                        }
                    }
                }

                total += function(&seen_this_iteration) * seen_this_iteration.len() as u32;
                seen.extend(seen_this_iteration);
            }
        }

        total
    }
}
