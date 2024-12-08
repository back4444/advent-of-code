pub mod shared {
    pub struct Question {
        pub value: u64,
        pub nums: Vec<u64>,
    }

    impl Question {
        pub fn check_combinations(&self, concat: bool) -> bool {
            self.check(1, self.nums[0], concat)
        }

        pub fn check(&self, i: usize, current_value: u64, concat: bool) -> bool {
            if i == self.nums.len() {
                return current_value == self.value;
            }

            let next_value = self.nums[i];

            let result = self.check(i + 1, current_value + next_value, concat)
                || self.check(i + 1, current_value * next_value, concat);

            if concat {
                result || self.check(i + 1, concat_unchecked(current_value, next_value), concat)
            } else {
                result
            }
        }
    }

    pub fn run_part(input: &str, concat: bool) -> u64 {
        input
            .lines()
            .filter_map(|line| {
                let (a, b) = line.split_once(":").unwrap();

                let question = Question {
                    value: a.parse().unwrap(),
                    nums: b.split_whitespace().map(|s| s.parse().unwrap()).collect(),
                };

                match question.check_combinations(concat) {
                    true => Some(question.value),
                    false => None,
                }
            })
            .sum()
    }

    fn concat_unchecked(a: u64, b: u64) -> u64 {
        format!("{}{}", a, b).parse().unwrap()
    }
}
