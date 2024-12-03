use day_03::shared;

fn main() {
    let result = run_part(include_str!("../input.txt"));
    println!("Result for day-03 part-1 == {}", result)
}

fn run_part(input: &str) -> u32 {
    let chars: Vec<char> = input.chars().collect();

    let mut total = 0;

    for i in 0..chars.len() {
        total += shared::check_mul(i, &chars, true)
    }

    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let result = run_part(include_str!("../test-1.txt"));

        assert_eq!(result, 161)
    }
}
