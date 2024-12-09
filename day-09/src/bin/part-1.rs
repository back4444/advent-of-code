use std::time::Instant;

fn main() {
    let instant = Instant::now();
    let result = run_part(include_str!("../input.txt"));
    let duration = instant.elapsed();
    println!("Result for day-09 part-1 == {}", result);
    println!("Took about {:?}", duration);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    File(u64),
    Empty,
}

fn run_part(input: &str) -> u64 {
    let mut total = 0;

    for line in input.lines() {
        let mut blocks: Vec<Block> = vec![];
        let mut next_id = 0;

        for (i, ch) in line.char_indices() {
            let size = ch.to_digit(10).unwrap();

            if i % 2 == 0 {
                for _ in 0..size {
                    blocks.push(Block::File(next_id));
                }
                next_id += 1;
            } else {
                for _ in 0..size {
                    blocks.push(Block::Empty);
                }
            }
        }

        for j in 0..blocks.len() {
            let last = blocks.len() - 1 - j;

            for k in 0..last {
                if blocks[k] == Block::Empty && blocks[last] != Block::Empty {
                    blocks[k] = blocks[last];
                    blocks[last] = Block::Empty;
                    break;
                }
            }
        }

        for (pos, block) in blocks.iter().enumerate() {
            if let Block::File(id) = block {
                total += id * pos as u64;
            }
        }
    }

    total
}

#[allow(unused)]
fn print_blocks(blocks: &[Block]) {
    for x in blocks {
        match x {
            Block::Empty => print!("."),
            Block::File(id) => print!("{id}"),
        }
    }
    println!()
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
