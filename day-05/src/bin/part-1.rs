use day_05::shared;

fn main() {
    let result = run_part(include_str!("../input.txt"));
    println!("Result for day-05 part-1 == {}", result)
}

fn run_part(input: &str) -> u32 {
    let (pairs, updates) = shared::parse_pairs_and_updates(input);

    updates
        .iter()
        .filter(|update| update.is_sorted_by(|&a, &b| pairs.contains(&(a, b))))
        .map(|update| update[(update.len() - 1) / 2])
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let result = run_part(&include_str!("../test.txt").replace("\r", ""));

        assert_eq!(result, 143)
    }
}
