use std::iter::zip;

use itertools::{Either, Itertools};

fn main() {
    part1();
}

type Key = [usize; 5];
type Lock = [usize; 5];

fn parse_entry(s: &str) -> Either<Key, Lock> {
    let mut entry = [0_usize; 5];

    s.lines()
        .skip(1)
        .take(5)
        .flat_map(|line| line.chars().enumerate())
        .filter(|(_, c)| c == &'#')
        .for_each(|(i, _)| entry[i] += 1);

    if s.starts_with('#') {
        Either::Right(entry)
    } else {
        Either::Left(entry)
    }
}

fn parse() -> (Vec<Key>, Vec<Lock>) {
    let data = std::fs::read_to_string("input/25.txt").unwrap();
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for entry in data.split("\n\n") {
        match parse_entry(entry) {
            Either::Left(key) => keys.push(key),
            Either::Right(lock) => locks.push(lock),
        }
    }
    (keys, locks)
}

fn part1() {
    let (keys, locks) = parse();
    let mut count = 0;
    for (key, lock) in keys.iter().cartesian_product(&locks) {
        if zip(key, lock).all(|(k, l)| k + l <= 5) {
            count += 1;
        }
    }
    println!("{count}");
}
