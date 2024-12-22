// doing some funky stuff on input set is not a good idea
// the random number contains all 24 bit numbers
// so we have overlapping regions for all the numbers due to pigeon hole principle
// but we cannot get a str1405ucture out of it
// So I just used hashsets to solve it in way shorter time than I ever imagined
// it helps that there are at most ~40_000 possible combos of diffs
// so O(2000 * 2000 + 2000 * 40_000) is still in the easily computable range.
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
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= 2001 {
            None
        } else {
            let out = self.num;
            self.count += 1;
            self.num = next_secret(out);
            Some(out)
        }
    }
}

fn next_secret(mut s: u32) -> u32 {
    let mask = (1 << 24) - 1;
    s = ((s << 6) ^ s) & mask;
    s = (s >> 5) ^ s;
    s = ((s << 11) ^ s) & mask;
    return s;
}

fn part2() {
    let nums = parse();
    let mut combos: HashMap<(u8, u8, u8, u8), usize> = HashMap::new();
    for num in nums.iter() {
        let mut num_combos: HashMap<(u8, u8, u8, u8), u8> = HashMap::new();
        let mut it = IterSecret {
            num: *num,
            count: 0,
        };
        let mut p0: u8;
        let mut p1: u8 = (it.next().unwrap() % 10).try_into().unwrap();
        let mut p2: u8 = (it.next().unwrap() % 10).try_into().unwrap();
        let mut p3: u8 = (it.next().unwrap() % 10).try_into().unwrap();
        let mut p4: u8 = (it.next().unwrap() % 10).try_into().unwrap();
        for new in it {
            p0 = p1;
            p1 = p2;
            p2 = p3;
            p3 = p4;
            p4 = (new % 10).try_into().unwrap();
            let d1: u8 = 9 + p1 - p0;
            let d2: u8 = 9 + p2 - p1;
            let d3: u8 = 9 + p3 - p2;
            let d4: u8 = 9 + p4 - p3;
            num_combos.entry((d1, d2, d3, d4)).or_insert(p4);
            // let e = num_combos.entry((d1, d2, d3, d4)).or_default();
            // *e = (*e).max(p4);
        }
        for (k, v) in num_combos {
            let e = combos.entry(k).or_default();
            *e = *e + usize::from(v);
        }
    }
    let mut max = 0;
    for (_k, v) in combos {
        max = max.max(v);
    }
    println!("{}", max);
}

fn part1() {
    let sum: u64 = parse()
        .into_iter()
        .map(|n| (0..2000).fold(n, |acc, _| next_secret(acc)))
        .map(u64::from)
        .sum();
    println!("{sum}");
}
