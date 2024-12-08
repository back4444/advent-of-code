use day_07::shared;

fn main() {
    let result = run_part(include_str!("../input.txt"));
    println!("Result for day-07 part-2 == {}", result)
}

fn run_part(input: &str) -> u64 {
    shared::run_part(input, true)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let result = run_part(include_str!("../test.txt"));

        assert_eq!(result, 11387)
    }
}
