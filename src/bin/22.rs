use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn parse() -> Vec<u32> {
    let data = std::fs::read_to_string("input/22.txt").unwrap();
    data.lines().map(|line| line.parse().unwrap()).collect()
}

struct IterSecret {
    count: usize,
    num: u32,
}

impl Iterator for IterSecret {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= 2001 {
            None
        } else {
            let out = self.num;
            self.count += 1;
            self.num = next_secret(out);
            Some((out % 10).try_into().unwrap())
        }
    }
}

fn next_secret(mut s: u32) -> u32 {
    let mask = (1 << 24) - 1;
    s = ((s << 6) ^ s) & mask;
    s = (s >> 5) ^ s;
    ((s << 11) ^ s) & mask
}

fn part2() {
    let nums = parse();
    let mut combos = HashMap::new();
    for num in nums.iter() {
        let mut num_combos = HashMap::new();
        let it = IterSecret {
            num: *num,
            count: 0,
        };
        for (a, b, c, d, e) in it.tuple_windows() {
            num_combos
                .entry((9 + b - a, 9 + c - b, 9 + d - c, 9 + e - d))
                .or_insert(e);
        }
        for (k, v) in num_combos {
            let e: &mut usize = combos.entry(k).or_default();
            *e += usize::from(v);
        }
    }
    println!("{}", combos.values().max().unwrap());
}

fn part1() {
    let sum: u64 = parse()
        .into_iter()
        .map(|n| (0..2000).fold(n, |acc, _| next_secret(acc)))
        .map(u64::from)
        .sum();
    println!("{sum}");
}
