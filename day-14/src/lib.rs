pub mod shared {
    use lib::point::Point;

    pub const GRID_X: i32 = 101;
    pub const GRID_Y: i32 = 103;

    pub fn parse_positions(input: &str) -> Vec<(Point, Point)> {
        input
            .lines()
            .map(|line| {
                let nums: Vec<i32> = line
                    .split_whitespace()
                    .flat_map(|part| {
                        part.split(',')
                            .flat_map(|s| s.split('=').last().unwrap().parse())
                    })
                    .collect();
                (Point::new(nums[0], nums[1]), Point::new(nums[2], nums[3]))
            })
            .collect()
    }
}
