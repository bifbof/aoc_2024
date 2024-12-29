// okay today felt quite nice.
// especially as the borrow checker works nicely here
use std::collections::HashMap;
type Cache = HashMap<(usize, usize), usize>;

fn main() {
    part1();
}

fn parse() -> Vec<usize> {
    let data = std::fs::read_to_string("input/11.txt").unwrap();
    data.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part1() {
    let nums = parse();
    let mut cache: Cache = HashMap::new();
    let mut count = 0;
    for val in nums {
        count += dfs(val, 75, &mut cache);
    }
    println!("{count}");
}

fn dfs(num: usize, steps: usize, cache: &mut Cache) -> usize {
    if cache.contains_key(&(num, steps)) {
        return cache[&(num, steps)];
    }
    let val = if steps == 0 {
        1 // arrived at end
    } else if num == 0 {
        dfs(1, steps - 1, cache)
    } else if (num.ilog10() + 1) & 1 == 0 {
        let middle = (num.ilog10() + 1) / 2;
        let offset = 10_usize.pow(middle);
        dfs(num / offset, steps - 1, cache) + dfs(num % offset, steps - 1, cache)
    } else {
        dfs(num * 2024, steps - 1, cache)
    };
    cache.insert((num, steps), val);
    val
}
