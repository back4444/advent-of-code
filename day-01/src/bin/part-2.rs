fn main() {
    let result = run_part(include_str!("../input.txt"));
    println!("Result for day-01 part-2 == {}", result)
}

fn run_part(input: &str) -> u32 {
    let (x, y): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let mut nums = line.split_whitespace().flat_map(str::parse::<u32>);
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip();

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
