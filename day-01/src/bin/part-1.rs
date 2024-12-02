use day_01::shared;

fn main() {
    let result = run_part(include_str!("../input.txt"));
    println!("Result for day-01 part-1 == {}", result)
}

fn run_part(input: &str) -> u32 {
    let (mut x, mut y) = shared::parse_lists(input);

    x.sort_unstable();
    y.sort_unstable();

    x.iter().zip(y).map(|(a, b)| a.abs_diff(b)).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let result = run_part(include_str!("../test.txt"));

        assert_eq!(result, 11)
    }
}
