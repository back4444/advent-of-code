// This works, but is not the most efficient. The index is not
// moved along each time a mul is found, meaning more checks
// are run than necessary . . .

pub mod shared {
    pub fn check_str(i: usize, vec: &[char], str: &str) -> bool {
        vec.get(i..i + str.len())
            .map_or(false, |slice| slice.iter().collect::<String>() == str)
    }

    pub fn check_mul(i: usize, vec: &[char], should_mul: bool) -> u32 {
        if !check_str(i, vec, "mul(") {
            return 0;
        }

        let mut buf = String::new();
        let mut j = i + 3;

        loop {
            match vec[j + 1] {
                next if next.is_ascii_digit() || next == ',' => buf.push(next),
                ')' => {
                    return match should_mul {
                        true => buf.split(',').flat_map(str::parse::<u32>).product(),
                        false => 0,
                    }
                }
                _ => return 0,
            }

            j += 1;
        }
    }
}
