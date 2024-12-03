use day_03::shared;

fn main() {
    let result = run_part(include_str!("../input.txt"));
    println!("Result for day-03 part-2 == {}", result)
}

fn run_part(input: &str) -> u32 {
    let chars: Vec<char> = input.chars().collect();

    let mut should_mul = true;
    let mut total = 0;

    for i in 0..chars.len() {
        if shared::check_str(i, &chars, "do()") {
            should_mul = true;
        } else if shared::check_str(i, &chars, "don't()") {
            should_mul = false
        }

        total += shared::check_mul(i, &chars, should_mul)
    }

    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let result = run_part(include_str!("../test-2.txt"));

        assert_eq!(result, 48)
    }
}
