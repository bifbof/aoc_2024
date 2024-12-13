use std::collections::HashMap;
use std::str::Lines;

type Graph = HashMap<usize, Vec<usize>>;

fn main() {
    part1();
    part2();
}

fn parse() -> (Graph, Vec<Vec<usize>>) {
    let data = std::fs::read_to_string("input/05.txt").expect("no read file");
    let mut iter = data.lines();
    let graph = parse_graph(&mut iter);
    let data = parse_data(&mut iter);
    (graph, data)
}

fn parse_graph(lines: &mut Lines) -> Graph {
    let mut graph = Graph::new();
    for line in lines {
        // if we encounter the dividing line break
        if line.is_empty() {
            break;
        }
        // fuck error handling
        let (key, val) = line.split_once('|').unwrap();
        let key = key.parse().unwrap();
        let val = val.parse().unwrap();
        graph
            .entry(key)
            .or_default()
            .push(val);
    }
    graph
}

fn parse_data(lines: &mut Lines) -> Vec<Vec<usize>> {
    let mut data = Vec::new();
    for line in lines {
        let line = line
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        data.push(line);
    }
    data
}

fn intersection(v1: &[usize], v2: &[usize]) -> Option<(usize, usize)> {
    for (i1, val1) in v1.iter().enumerate() {
        for (i2, val2) in v2.iter().enumerate() {
            if val1 == val2 {
                return Some((i1, i2));
            }
        }
    }
    None
}

fn is_topologically_sorted(update: &[usize], graph: &Graph) -> Option<(usize, usize)> {
    let mut prev = Vec::new();
    for (idx_val, val) in update.iter().enumerate() {
        prev.push(*val);
        let neighbors = match graph.get(val) {
            None => continue,
            Some(v) => v,
        };
        match intersection(&prev, neighbors) {
            None => {}
            Some((idx, _)) => return Some((idx, idx_val)),
        }
    }
    None
}

fn part1() {
    let (g, d) = parse();
    let mut count_good = 0;
    for line in d {
        if is_topologically_sorted(&line, &g).is_none() {
            count_good += line[line.len() / 2];
        }
    }
    println!("{}", count_good);
}

fn part2() {
    let (g, d) = parse();
    let mut count = 0;
    for mut line in d {
        // if correctly sorted at beginning skip
        if is_topologically_sorted(&line, &g).is_none() {
            continue;
        }
        // as multiple topological sortings are possible,
        // I correct only the ones that are wrong
        while let Some((i1, i2)) = is_topologically_sorted(&line, &g) {
            line.swap(i1, i2);
        }
        count += line[line.len() / 2]
    }
    println!("{}", count);
}
