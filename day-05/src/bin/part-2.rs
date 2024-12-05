use day_05::shared;

fn main() {
    let result = run_part(include_str!("../input.txt"));
    println!("Result for day-05 part-2 == {}", result)
}

fn run_part(input: &str) -> u32 {
    let (pairs, updates) = shared::parse_pairs_and_updates(input);

    let mut count = 0;

    for mut update in updates {
        let n = update.len();

        let mut was_incorrect = false;

        for i in 0..n {
            for j in 0..n - i - 1 {
                if !pairs.contains(&(update[j], update[j + 1])) {
                    was_incorrect = true;

                    update.swap(j, j + 1);
                }
            }
        }

        if was_incorrect {
            count += update[n / 2]
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let result = run_part(&include_str!("../test.txt").replace("\r", ""));

        assert_eq!(result, 123)
    }
}
