pub fn part1(input: &str) -> u64 {
    let nums: Vec<_> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    let blocks = nums
        .iter()
        .step_by(2)
        .enumerate()
        .map(|(i, n)| (0..*n).map(move |_| i));

    let mut rev_blocks = blocks.clone().flatten().rev();
    let total_nums: usize = rev_blocks.clone().count();

    let frees = nums.iter().skip(1).step_by(2);
    let filesystem = blocks.zip(frees);

    let compressed = filesystem
        .flat_map(|(block, free)| {
            let size = *free as usize;
            let taken: Vec<_> = rev_blocks.by_ref().take(size).collect();
            block.chain(taken)
        })
        .take(total_nums);

    compressed
        .enumerate()
        .map(|(i, n)| (i * n) as u64)
        .sum::<u64>()
}

pub fn part2(input: &str) -> u64 {
    let nums: Vec<_> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let rev_blocks = nums
        .iter()
        .step_by(2)
        .enumerate()
        .rev();

    let mut frees: Vec<_> = nums.clone().into_iter().skip(1).step_by(2).collect();

    // This is a vector of vectors, representing a bucket where blocks are inserted.
    // in the end, the vector is flattened and the resulting vector is obtained.
    let mut indexed = vec![vec![]; frees.len()];

    // starting at the back, check if there is a free space for that block
    // if there is, then move it there
    // otherwise, we cannot move the block and the idx is frozen
    for (rev_block, rev_size) in rev_blocks{
        let my_idx = rev_block;
        let free_idx = frees.iter().take(my_idx).position(|free| free >= rev_size);
        if let Some(free_idx) = free_idx {
            // found a free idx, let's occupy this space
            let free = frees[free_idx];
            frees[free_idx] = free - *rev_size;

            // now we can move the block to the found idx
            indexed[free_idx].push((rev_size, rev_block));

            // and make the spot free again
            frees[my_idx - 1] += rev_size;
            continue;
        }

        // Otherwise, we have found an element that cannot be moved
        // in that case, we must move in to the front of the idx, as it takes priority
        indexed[my_idx].insert(0, (rev_size, rev_block));
    }

    // all we have to do is insert 0's in the places where we still have empty values
    let mut result: u64 = 0;
    let mut factor = 0;
    for (num_free, items) in frees.iter().zip(indexed) {
        for (size, item) in items {
            let size = *size;
            let sum = (size * item * (2 * factor + size - 1)) / 2;
            result += sum as u64;
            factor += size;
        }
        factor += num_free;
    }

    return result;
}
