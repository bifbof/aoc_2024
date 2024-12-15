// omg is my solution overcomplicated
// it is quite simpler.
// basically the same as part 1,
// two pointer, if you can place it insert it and make other invlid
use std::collections::VecDeque;
use std::mem::swap;

fn main() {
    part1();
    part2();
}

fn parse() -> Vec<usize> {
    let data = std::fs::read_to_string("input/09.txt").expect("File read error");
    data.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
        .collect()
}

enum Fill {
    Some,
    None,
}

fn part1() {
    let data = parse();
    let total_len = data.iter().sum();
    let mut disk: Vec<Option<usize>> = vec![None; total_len];

    let mut num = 0;
    let mut idx = 0;
    for (&len, op) in data.iter().zip([Fill::Some, Fill::None].iter().cycle()) {
        if matches!(op, Fill::None) {
            idx += len;
            continue;
        }
        for _ in 0..len {
            disk[idx] = Some(num);
            idx += 1;
        }
        num += 1;
    }
    let mut left = 0;
    let mut right = total_len - 1;
    while left < right {
        if disk[left].is_some() {
            left += 1;
            continue;
        }
        if disk[right].is_none() {
            right -= 1;
            continue;
        }
        let (disk_left, disk_right) = disk.split_at_mut(right);
        swap(&mut disk_left[left], &mut disk_right[0]);
    }
    println!();
    let c = disk
        .iter()
        .enumerate()
        .map(|(i, v)| v.map_or(0, |v| v * i))
        .sum::<usize>();
    println!("{c}");
}

#[derive(Debug, Clone)]
struct Block {
    value: usize,
    len: usize,
    free: usize,
}

fn part2() {
    let data = parse().into_iter();
    let mut value = 0;
    let mut blocks: VecDeque<Block> = VecDeque::new();

    for (len, op) in data.zip([Fill::Some, Fill::None].iter().cycle()) {
        match op {
            Fill::None => blocks.back_mut().unwrap().free = len,
            Fill::Some => {
                blocks.push_back(Block {
                    value,
                    len,
                    free: 0,
                });
                value += 1;
            }
        }
    }
    let inserts = blocks.clone();
    let mut blocks_new: VecDeque<Block> = VecDeque::new();


    // I do the ownership trick with two queues [...] + elem + [...]
    for mut insert in inserts.into_iter().rev() {
        let mut inserted = false;
        // println!("{:?}", blocks);
        while let Some(mut block) = blocks.pop_front() {
            if block.value == insert.value {
                // inserted between blocks -> update free
                if inserted {
                    // println!("removed block {:?}", insert.value, insert.len, insert.free, insert.len + insert.free);
                    blocks_new.back_mut().unwrap().free += block.len + block.free;
                } else {
                    blocks_new.push_back(block);
                }
                inserted = true;
            } else if !inserted && block.free >= insert.len {
                insert.free = block.free - insert.len;
                block.free = 0;
                blocks_new.push_back(block);
                blocks_new.push_back(insert.clone());
                inserted = true;
            } else {
                blocks_new.push_back(block);
            }
        }

        swap(&mut blocks, &mut blocks_new);
    }
    let mut idx = 0;
    let mut count = 0;
    for block in blocks.iter() {
        for _ in 0..block.len {
            count += idx * block.value;
            idx += 1;
        }
        idx += block.free;
    }
    println!("{count}");
}
