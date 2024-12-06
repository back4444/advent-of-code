use day_06::shared;

fn main() {
    let result = run_part(include_str!("../input.txt"));
    println!("Result for day-06 part-1 == {}", result)
}

fn run_part(input: &str) -> u32 {
    let (grid, guard_pos) = shared::parse_grid_and_guard_pos(input);
    shared::calculate_visited(&grid, guard_pos).len() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let result = run_part(include_str!("../test.txt"));

        assert_eq!(result, 41)
    }
}
