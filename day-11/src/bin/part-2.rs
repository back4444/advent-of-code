use std::time::Instant;

use day_11::shared;

fn main() {
    let instant = Instant::now();
    let result = run_part(include_str!("../input.txt"));
    let duration = instant.elapsed();
    println!("Result for day-11 part-2 == {}", result);
    println!("Took about {:?}", duration);
}

fn run_part(input: &str) -> u64 {
    shared::count_stones(input, 75)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let result = run_part(include_str!("../test.txt"));

        assert_eq!(result, 65601038650482)
    }
}
