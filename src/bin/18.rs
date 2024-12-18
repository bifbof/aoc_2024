use itertools::Itertools;
use std::{collections::VecDeque, iter::zip};

type Graph = Vec<Vec<usize>>;

fn main() {
    solve();
}

fn parse() -> Vec<(usize, usize)> {
    let data = std::fs::read_to_string("input/18.txt").unwrap();
    let mut out = Vec::new();
    for line in data.lines() {
        let (col, row) = line.split_once(',').unwrap();
        let row = row.parse().unwrap();
        let col = col.parse().unwrap();
        out.push((row, col));
    }
    out
}

fn shortest_path(adj: &Graph, start: usize, goal: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..adj.len()).map(|_| usize::MAX).collect();
    let mut queue = VecDeque::from([start]);

    dist[start] = 0;
    while let Some(node) = queue.pop_front() {
        if node == goal {
            return Some(dist[goal]);
        }
        for next in &adj[node] {
            let cost = dist[node] + 1;
            if cost < dist[*next] {
                queue.push_back(*next);
                dist[*next] = cost;
            }
        }
    }
    None
}

fn get_dist(size: usize, steps: usize, blocks: &[(usize, usize)]) -> Option<usize> {
    let start = (0, 0);
    let goal = (size - 1, size - 1);
    let mut grid = vec![vec![true; size]; size];
    for (_i, (r, c)) in zip(0..steps, blocks) {
        grid[*r][*c] = false;
    }

    let mut graph: Graph = vec![Vec::new(); size * size];

    for (r, c) in (0..size).cartesian_product(0..size) {
        let idx = r * size + c;
        if r > 0 && grid[r - 1][c] {
            let other = (r - 1) * size + c;
            graph[idx].push(other);
        }
        if r < (size - 1) && grid[r + 1][c] {
            let other = (r + 1) * size + c;
            graph[idx].push(other);
        }
        if c > 0 && grid[r][c - 1] {
            let other = r * size + c - 1;
            graph[idx].push(other);
        }
        if c < (size - 1) && grid[r][c + 1] {
            let other = r * size + c + 1;
            graph[idx].push(other);
        }
    }
    shortest_path(&graph, start.0 * size + start.1, goal.0 * size + goal.1)
}

fn solve() {
    let size = 71;
    let blocks = parse();
    let d = get_dist(size, 1024, &blocks).unwrap();
    println!("{d}");

    let mut left = 0;
    let mut right = blocks.len();
    while left < right {
        let mid = (left + right) / 2;
        match get_dist(size, mid, &blocks) {
            None => right = mid,
            _ => left = mid + 1,
        }
    }
    // everything left of left (excluding) is Some()
    // everything right of right (including) is None
    let b = blocks[left - 1];
    println!("{},{}", b.1, b.0);
}
