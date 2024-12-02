pub mod shared {
    pub fn parse_reports(input: &str) -> Vec<Vec<i32>> {
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .flat_map(str::parse::<i32>)
                    .collect()
            })
            .collect()
    }

    pub fn check_report(report: &[i32]) -> bool {
        let mut max = 0;
        let (mut neg, mut zero, mut pos) = (false, false, false);

        for d in report.windows(2).map(|w| w[1] - w[0]) {
            match d.cmp(&0) {
                std::cmp::Ordering::Less => neg = true,
                std::cmp::Ordering::Equal => zero = true,
                std::cmp::Ordering::Greater => pos = true,
            }

            if (neg && pos) || zero {
                return false;
            }

            max = max.max(d.abs());
        }

        (1..=3).contains(&max)
    }
}
