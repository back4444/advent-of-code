use day_04::shared;

fn main() {
    let result = run_part(include_str!("../input.txt"));
    println!("Result for day-04 part-2 == {}", result)
}

#[rustfmt::skip]
const UNIT_VECTORS: [[(i32, i32); 2]; 2] = [[(-1, 1), (1, -1)], [(1, 1), (-1, -1)]];

fn run_part(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let n = grid.len();

    let mut total = 0;

    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 'A' {
                let mut correct_parts = 0;

                for unit in UNIT_VECTORS {
                    let mut buf = String::new();

                    for (di, dj) in unit {
                        let k_di = i as i32 + di;
                        let k_dj = j as i32 + dj;

                        if shared::in_grid(k_di, k_dj, n) {
                            buf.push(grid[k_di as usize][k_dj as usize]);
                        }
                    }

                    if buf == "MS" || buf == "SM" {
                        correct_parts += 1;
                    }
                }

                if correct_parts == 2 {
                    total += 1;
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

        assert_eq!(result, 9)
    }
}
