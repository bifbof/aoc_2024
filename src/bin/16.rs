// went smoothly, my graph creation is way too long
// so I thought fuck it and made the backtracking as ugly as it is. :)
use itertools::Itertools;
use std::{
    collections::{BinaryHeap, HashSet},
    ops::Deref,
};

type Graph = Vec<Vec<Edge>>;
type Start = (usize, usize);
type Goal = (usize, usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Edge {
    node: usize,
    cost: usize,
}

impl Edge {
    const fn new(node: usize, cost: usize) -> Self {
        Self { node, cost }
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}
impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Grid<T> {
    nrows: usize,
    ncols: usize,
    data: Vec<Vec<T>>,
}

impl<T> Deref for Grid<T> {
    type Target = Vec<Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

fn main() {
    solve();
}

fn parse() -> (Grid<char>, Start, Goal) {
    let data = std::fs::read_to_string("input/16.txt").unwrap();
    let data: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let nrows = data.len();
    let ncols = data.first().unwrap().len();
    let start = (0..nrows)
        .cartesian_product(0..ncols)
        .find(|(r, c)| data[*r][*c] == 'S')
        .unwrap();
    let goal = (0..nrows)
        .cartesian_product(0..ncols)
        .find(|(r, c)| data[*r][*c] == 'E')
        .unwrap();
    (Grid { nrows, ncols, data }, start, goal)
}

fn shortest_path(adj: &Graph, start: usize, goal: usize) -> Vec<usize> {
    let mut dist: Vec<_> = (0..adj.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(Edge::new(start, 0));
    while let Some(Edge { node, cost }) = heap.pop() {
        // print_edge_info(& Edge::new(node, cost));
        if node == goal {
            break;
        }
        if cost > dist[node] {
            continue;
        }
        for edge in &adj[node] {
            let next = Edge::new(edge.node, cost + edge.cost);
            if next.cost < dist[edge.node] {
                heap.push(next);
                dist[next.node] = next.cost;
            }
        }
    }

    dist
}

fn rev_graph(old: &Graph) -> Graph {
    let mut graph: Graph = vec![Vec::new(); old.len()];
    for (from, edges) in old.iter().enumerate() {
        for edge in edges {
            let mut edge = *edge;
            let to = edge.node;
            edge.node = from;
            graph[to].push(edge);
        }
    }
    graph
}

fn solve() {
    let (grid, s, g) = parse();
    let nodes = grid.nrows * grid.ncols;
    let mut graph: Graph = vec![Vec::new(); nodes * 4];
    // build the graph
    let offset_north = 0;
    let offset_west = nodes;
    let offset_south = 2 * nodes;
    let offset_east = 3 * nodes;
    // could have been done smarter but copy past is fast
    for (row, col) in (0..grid.nrows).cartesian_product(0..grid.ncols - 1) {
        if grid[row][col] == '#' {
            continue;
        }
        // first north facing graph
        let idx = offset_north + col + grid.ncols * row;
        if grid[row - 1][col] != '#' {
            graph[idx].push(Edge::new(offset_north + col + grid.ncols * (row - 1), 1));
        }
        graph[idx].push(Edge::new(idx - offset_north + offset_east, 1000));
        graph[idx].push(Edge::new(idx - offset_north + offset_west, 1000));

        // then west
        let idx = offset_west + col + grid.ncols * row;
        if grid[row][col + 1] != '#' {
            graph[idx].push(Edge::new(offset_west + (col + 1) + grid.ncols * row, 1));
        }
        graph[idx].push(Edge::new(idx - offset_west + offset_north, 1000));
        graph[idx].push(Edge::new(idx - offset_west + offset_south, 1000));

        // then south
        let idx = offset_south + col + grid.ncols * row;
        if grid[row + 1][col] != '#' {
            graph[idx].push(Edge::new(offset_south + col + grid.ncols * (row + 1), 1));
        }
        graph[idx].push(Edge::new(idx - offset_south + offset_east, 1000));
        graph[idx].push(Edge::new(idx - offset_south + offset_west, 1000));

        // then east
        let idx = offset_east + col + grid.ncols * row;
        if grid[row][col - 1] != '#' {
            graph[idx].push(Edge::new(offset_east + (col - 1) + grid.ncols * row, 1));
        }
        graph[idx].push(Edge::new(idx - offset_east + offset_north, 1000));
        graph[idx].push(Edge::new(idx - offset_east + offset_south, 1000));
    }
    // fix the end such that we have only one
    let goal_idx = g.1 + g.0 * grid.ncols;
    graph[goal_idx + offset_north].push(Edge::new(goal_idx + offset_east, 0));
    graph[goal_idx + offset_west].push(Edge::new(goal_idx + offset_east, 0));
    graph[goal_idx + offset_south].push(Edge::new(goal_idx + offset_east, 0));

    let start = s.1 + s.0 * grid.ncols + offset_east;
    let goal = g.1 + g.0 * grid.ncols + offset_east;
    let dist = shortest_path(&graph, start, goal);
    println!("{}", dist[goal]);

    // and then just dfs backwards
    let graph = rev_graph(&graph);
    let mut visited = HashSet::new();
    let mut stack = vec![goal];

    while let Some(node) = stack.pop() {
        let n = node % nodes;
        visited.insert((n / grid.nrows, n % grid.nrows));
        if dist[node] == 0 {
            continue;
        }
        for neigh in &graph[node] {
            if dist[neigh.node] == (dist[node] - neigh.cost) {
                stack.push(neigh.node);
            }
        }
    }
    println!("{}", visited.len());
}
