use std::fs;

fn main() {
    part1();
    part2();
}

fn parse() -> Vec<Vec<usize>> {
    let filename = "input/02.txt";
    let data = fs::read_to_string(filename).expect("Unable to read file");

    let mut v = Vec::new();
    for line in data.lines() {
        let mut linev = Vec::new();
        for s in line.split_whitespace() {
            linev.push(s.parse::<usize>().unwrap());
        }
        v.push(linev);
    }
    v
}

fn part1() {
    let matrix = parse();
    let mut num_safe = 0;
    for row in matrix {
        if is_safe(&row) {
            num_safe += 1;
        }
    }
    println!("safe: {}", num_safe);
}

fn is_safe(row: &[usize]) -> bool {
    // check three things:
    let mut decreasing = true;
    let mut increasing = true;
    let mut enough_dist = true;
    for i in 0..(row.len() - 1) {
        decreasing = decreasing && (row[i] > row[i + 1]);
        increasing = increasing && (row[i] < row[i + 1]);
        // check that at least one is done by increasing/decreasing
        enough_dist = enough_dist && (row[i].abs_diff(row[i + 1]) < 4)
    }
    (decreasing || increasing) && enough_dist
}

fn part2() {
    let matrix = parse();
    let mut num_safe = 0;
    for row in matrix {
        if is_damper_safe(row) {
            num_safe += 1;
        }
    }
    println!("safe: {}", num_safe);
}

fn is_damper_safe(row: Vec<usize>) -> bool {
    for skip_idx in 0..row.len() {
        let new_row: Vec<usize> = row
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != skip_idx)
            .map(|(_, val)| *val)
            .collect();
        if is_safe(&new_row) {
            return true;
        }
    }
    false
}