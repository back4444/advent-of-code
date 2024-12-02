pub mod shared {
    pub fn parse_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
        input
            .lines()
            .map(|line| {
                let mut nums = line.split_whitespace().flat_map(str::parse::<u32>);
                (nums.next().unwrap(), nums.next().unwrap())
            })
            .unzip()
    }
}
