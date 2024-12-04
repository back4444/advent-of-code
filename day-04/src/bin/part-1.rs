use day_04::shared;

fn main() {
    let result = run_part(include_str!("../input.txt"));
    println!("Result for day-04 part-1 == {}", result)
}

#[rustfmt::skip]
const UNIT_VECTORS: [(i32, i32); 8] = [ (1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1), ];

fn run_part(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let n = grid.len();

    let mut total = 0;

    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 'X' {
                for (di, dj) in UNIT_VECTORS {
                    let mut correct_letters = 0;

                    for (k, letter) in "MAS".char_indices() {
                        let k_di = i as i32 + (di * (k as i32 + 1));
                        let k_dj = j as i32 + (dj * (k as i32 + 1));

                        if shared::in_grid(k_di, k_dj, n)
                            && grid[k_di as usize][k_dj as usize] == letter
                        {
                            correct_letters += 1;
                        }
                    }

                    if correct_letters == 3 {
                        total += 1;
                    }
                }
            }
        }
    }

    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let result = run_part(include_str!("../test.txt"));

        assert_eq!(result, 18)
    }
}
