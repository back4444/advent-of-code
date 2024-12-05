pub mod shared {
    pub fn parse_pairs_and_updates(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
        let (section_one, section_two) = input.split_once("\n\n").unwrap();

        let pairs: Vec<(u32, u32)> = section_one
            .lines()
            .map(|line| {
                let (a, b) = line.split_once("|").unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect();

        let updates: Vec<Vec<u32>> = section_two
            .lines()
            .map(|line| line.split(",").map(|s| s.parse().unwrap()).collect())
            .collect();

        (pairs, updates)
    }
}
