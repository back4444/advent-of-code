pub mod shared {
    use lib::point::Point;

    #[derive(Debug, Clone)]
    pub struct Antenna {
        pub freq: char,
        pub pos: Point,
    }

    pub fn parse_antennas(input: &str) -> (Vec<Antenna>, usize) {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let n = grid.len();

        let mut antennas = vec![];

        #[allow(clippy::needless_range_loop)]
        for y in 0..n {
            for x in 0..n {
                let freq = grid[y][x];

                if freq != '.' {
                    antennas.push(Antenna {
                        freq,
                        pos: Point::new(x as i32, y as i32),
                    });
                }
            }
        }

        (antennas, n)
    }
}
