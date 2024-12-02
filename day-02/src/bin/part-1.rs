use day_02::shared;

fn main() {
    let result = run_part(include_str!("../input.txt"));
    println!("Result for day-02 part-1 == {}", result)
}

fn run_part(input: &str) -> u32 {
    shared::parse_reports(input)
        .iter()
        .filter(|report| shared::check_report(report))
        .count() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let result = run_part(include_str!("../test.txt"));

        assert_eq!(result, 2)
    }
}
