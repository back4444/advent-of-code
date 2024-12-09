use std::time::Instant;

fn main() {
    let instant = Instant::now();
    let result = run_part(include_str!("../input.txt"));
    let duration = instant.elapsed();
    println!("Result for day-09 part-1 == {}", result);
    println!("Took about {:?}", duration);
}

fn run_part(input: &str) -> u64 {
    let mut blocks: Vec<Option<u64>> = vec![];
    let mut next_id = 0;

    for (i, ch) in input.char_indices() {
        let size = ch.to_digit(10).unwrap();

        if i % 2 == 0 {
            for _ in 0..size {
                blocks.push(Some(next_id));
            }
            next_id += 1;
        } else {
            for _ in 0..size {
                blocks.push(None);
            }
        }
    }

    moves(&mut blocks);

    let mut total = 0;

    for (pos, block) in blocks.iter().enumerate() {
        if let Some(id) = block {
            total += id * pos as u64;
        }
    }

    total
}

fn moves(blocks: &mut [Option<u64>]) {
    let mut i = 0;
    let mut j = blocks.len() - 1;

    while i < j {
        if blocks[i].is_none() {
            blocks.swap(i, j);
            j -= 1
        } else {
            i += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part() {
        let result = run_part(include_str!("../test.txt"));

        assert_eq!(result, 1928)
    }
}
