pub mod shared {
    use std::collections::HashMap;

    fn split_in_half(num: u64, digits_count: u32) -> (u64, u64) {
        let divisor = 10_u64.pow(digits_count / 2);
        (num / divisor, num % divisor)
    }

    pub fn count_stones(input: &str, depth: usize) -> u64 {
        let mut stones: HashMap<u64, u64> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .fold(HashMap::new(), |mut acc, stone| {
                *acc.entry(stone).or_insert(0) += 1;
                acc
            });

        for _ in 0..depth {
            let mut current_stones = HashMap::new();

            for (stone, count) in stones.into_iter() {
                if stone == 0 {
                    current_stones
                        .entry(1)
                        .and_modify(|new_count| *new_count += count)
                        .or_insert(count);

                    continue;
                }

                let digits_count = stone.checked_ilog10().unwrap_or(0) + 1;

                if digits_count % 2 == 0 {
                    let (left, right) = split_in_half(stone, digits_count);

                    current_stones
                        .entry(left)
                        .and_modify(|new_count| *new_count += count)
                        .or_insert(count);

                    current_stones
                        .entry(right)
                        .and_modify(|new_count| *new_count += count)
                        .or_insert(count);

                    continue;
                }

                current_stones
                    .entry(stone * 2024)
                    .and_modify(|new_count| *new_count += count)
                    .or_insert(count);
            }

            stones = current_stones;
        }

        stones.values().sum()
    }
}
