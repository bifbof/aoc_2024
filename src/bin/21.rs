// for this DP I searched too long for a need recursive formula
// gave up and just checked all paths :c
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::repeat_n;
use Dir::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Dir {
    Activate,
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn goto(&self, other: &Self) -> Vec<Vec<Dir>> {
        if self == other {
            return vec![vec![Activate]];
        }
        match (self, other) {
            (Activate, Right) => vec![vec![Down, Activate]],
            (Activate, Down) => vec![vec![Down, Left, Activate], vec![Left, Down, Activate]],
            (Activate, Left) => vec![
                vec![Down, Left, Left, Activate],
                vec![Left, Down, Left, Activate],
            ],
            (Activate, Up) => vec![vec![Left, Activate]],
            (Right, Down) => vec![vec![Left, Activate]],
            (Right, Left) => vec![vec![Left, Left, Activate]],
            (Right, Up) => vec![vec![Up, Left, Activate], vec![Left, Up, Activate]],
            (Down, Left) => vec![vec![Left, Activate]],
            (Down, Up) => vec![vec![Up, Activate]],
            (Left, Up) => vec![vec![Right, Up, Activate]],
            _ => reverse(other.goto(self)),
        }
    }
}

fn reverse(mut paths: Vec<Vec<Dir>>) -> Vec<Vec<Dir>> {
    for path in &mut paths {
        path.pop();
        path.reverse();
        for val in &mut *path {
            *val = match val {
                Activate => unreachable!(),
                Up => Down,
                Down => Up,
                Right => Left,
                Left => Right,
            };
        }
        path.push(Activate);
    }
    paths
}

fn main() {
    // dbg!(numpad_goto('A', '1'));
    part1();
}

fn parse() -> Vec<String> {
    let data = std::fs::read_to_string("input/21.txt").unwrap();
    data.lines().map(|t| t.to_owned()).collect()
}

fn pos_numpad(c: char) -> (i8, i8) {
    match c {
        'A' => (3, 2),
        '0' => (3, 1),
        '1'..='9' => {
            let offset = (c as i32) - ('1' as i32);
            let row = 2 - offset / 3;
            let col = offset % 3;
            (row as i8, col as i8)
        }
        _ => unreachable!(),
    }
}

fn good_numpad_path(path: &[Dir], start: (i8, i8)) -> bool {
    let (mut r, mut c) = start;
    for dir in path {
        (r, c) = match dir {
            Activate => return true,
            Up => (r - 1, c),
            Right => (r, c + 1),
            Down => (r + 1, c),
            Left => (r, c - 1),
        };
        if (r, c) == (3, 0) {
            return false;
        }
    }
    true
}

fn numpad_goto(from: char, to: char) -> Vec<Vec<Dir>> {
    let (fx, fy) = pos_numpad(from);
    let (tx, ty) = pos_numpad(to);
    let (dx, dy) = (tx - fx, ty - fy);
    let mut path = Vec::new();
    if dy > 0 {
        path.extend(repeat_n(Right, dy.try_into().unwrap()));
    } else {
        path.extend(repeat_n(Left, dy.abs().try_into().unwrap()));
    }
    if dx > 0 {
        path.extend(repeat_n(Down, dx.try_into().unwrap()));
    } else {
        path.extend(repeat_n(Up, dx.abs().try_into().unwrap()));
    }
    let len = path.len();
    path.into_iter()
        .permutations(len)
        .unique()
        .filter(|t| good_numpad_path(t, (fx, fy)))
        .map(|mut p| {
            p.push(Activate);
            p
        })
        .collect()
}

fn dist(from: Dir, to: Dir, level: u8, cache: &mut HashMap<(Dir, Dir, u8), usize>) -> usize {
    if level == 0 {
        return 1;
    }
    let key = (from, to, level);
    if cache.contains_key(&key) {
        return cache[&key];
    }
    let mut min_cost = usize::MAX;
    for path in from.goto(&to) {
        let mut cost = 0;
        let mut curr = Activate;
        for next in path {
            cost += dist(curr, next, level - 1, cache);
            curr = next;
        }
        min_cost = min_cost.min(cost);
    }
    cache.insert(key, min_cost);
    min_cost
}

fn get_answer(s: &str) -> usize {
    let num: usize = s[..3].parse().unwrap();
    // let paths = parse_numpad(s);
    let mut cache = HashMap::new();
    let mut curr_numpad = 'A';
    let mut cost = 0;
    for next_numpad in s.chars() {
        if numpad_goto(curr_numpad, next_numpad).is_empty() {
            println!("{curr_numpad} to {next_numpad}");
        }
        let new_cost = numpad_goto(curr_numpad, next_numpad)
        .into_iter()
        .map(|path| {
            let mut curr = Activate;
            let mut cost = 0;
            for next in path {
                cost += dist(curr, next, 25, &mut cache);
                curr = next;
            }
            cost
        })
        .min()
        .unwrap();
    cost += new_cost;
    curr_numpad = next_numpad;
    }
    println!("{s}: {cost} * {num} = {}", cost * num);
    cost * num
}

fn part1() {
    let input = parse();
    let c: usize = input.iter().map(|i| get_answer(i)).sum();
    println!("{c}");
}
