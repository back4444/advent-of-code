use day_02::shared;

fn main() {
    let result = run_part(include_str!("../input.txt"));
    println!("Result for day-02 part-2 == {}", result)
}

fn run_part(input: &str) -> u32 {
    let reports = shared::parse_reports(input);

    let mut count = 0;

    for report in reports {
        if shared::check_report(&report) {
            count += 1;
        } else {
            for i in 0..report.len() - 1 {
                let mut modified_report = report.clone();
                modified_report.remove(i);

                if shared::check_report(&modified_report) {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let result = run_part(include_str!("../test.txt"));

        assert_eq!(result, 4)
    }
}
