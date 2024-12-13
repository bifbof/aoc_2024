use std::{fs, iter::zip};
use std::collections::HashMap;


fn main() {
    let filename = "input/01.txt";
    let data = fs::read_to_string(filename).expect("Unable to read file");
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();

    for line in data.lines() {
        let mut line = line.split_whitespace();
        let (Some(l), Some(r), None) = (line.next(), line.next(), line.next()) else {
            panic!();
        };
        left.push(l.parse().unwrap());
        right.push(r.parse().unwrap());
    }
    left.sort();
    right.sort();

    let distance: usize = zip(&left, &right).map(|(l, r)| l.abs_diff(*r)).sum();

    // okay with hashmap this looks a lot cleaner
    let mut hm: HashMap<usize, usize> = HashMap::new();

    for val in &left {
        hm.entry(*val).and_modify(|c| *c += 1).or_insert(1);
    }
    let mut similarity = 0;
    for val in &right {
        similarity += hm.get(val).unwrap_or(&0) * val;
    }
    println!("distance:   {:-10}", distance);
    println!("similarity: {:-10}", similarity);
}
