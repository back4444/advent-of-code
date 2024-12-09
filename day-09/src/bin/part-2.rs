use std::time::Instant;

fn main() {
    let instant = Instant::now();
    let result = run_part(include_str!("../input.txt"));
    let duration = instant.elapsed();
    println!("Result for day-09 part-2 == {}", result);
    println!("Took about {:?}", duration);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    File(u64, u64),
    Empty(u64),
}

fn run_part(input: &str) -> u64 {
    let mut blocks: Vec<Block> = vec![];
    let mut next_id = 0;

    for (i, ch) in input.char_indices() {
        let size = ch.to_digit(10).unwrap();

        if i % 2 == 0 {
            blocks.push(Block::File(next_id, size as u64));
            next_id += 1;
        } else {
            blocks.push(Block::Empty(size as u64));
        }
    }

    let mut id_to_move = next_id;

    'outer: while id_to_move > 0 {
        id_to_move -= 1;

        // Finds the file with the next id to move
        if let Some((i, &Block::File(_, f_size))) = blocks
            .iter()
            .enumerate()
            .find(|(_, block)| matches!(block, Block::File(id, _) if *id == id_to_move))
        {
            // Find the first Empty block that can contain the file.
            // If they are equal in size, just straight swap.
            // Otherwise, we decrease the Empty insert the file at the index of this Empty.
            for j in 0..i {
                if let Block::Empty(e_size) = blocks[j] {
                    if e_size >= f_size {
                        if e_size == f_size {
                            blocks[j] = blocks[i];
                            blocks[i] = Block::Empty(f_size);
                        } else {
                            let file = blocks[i];

                            blocks[j] = Block::Empty(e_size - f_size);
                            blocks[i] = Block::Empty(f_size);

                            blocks.insert(j, file);
                        }

                        continue 'outer;
                    }
                }
            }
        }
    }

    let mut total = 0;
    let mut index = 0;

    for block in blocks {
        match block {
            Block::File(id, size) => {
                for offset in 0..size {
                    total += (index + offset) * id;
                }
                index += size;
            }
            Block::Empty(size) => index += size,
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

        assert_eq!(result, 2858)
    }
}
