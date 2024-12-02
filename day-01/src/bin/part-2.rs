use day_01::shared;

fn main() {
    let result = run_part(include_str!("../input.txt"));
    println!("Result for day-01 part-2 == {}", result)
}

fn run_part(input: &str) -> u32 {
    let (x, y) = shared::parse_lists(input);

    x.iter()
        .map(|a| a * y.iter().filter(|&b| a == b).count() as u32)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let result = run_part(include_str!("../test.txt"));

        assert_eq!(result, 31)
    }
}
