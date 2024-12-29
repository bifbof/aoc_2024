use std::collections::{HashMap, BTreeSet};

use itertools::Itertools;

type Graph = Vec<BTreeSet<usize>>;

fn main() {
    solve();
}

fn parse() -> (Graph, Vec<String>) {
    let data = std::fs::read_to_string("input/23.txt").unwrap();
    let mut graph = Graph::new();
    let mut seen = HashMap::new();
    for (n0, n1) in data.lines().map(|s| s.split_once('-').unwrap()) {
        let [c0, c1] = [n0, n1].map(str::to_string);
        let [e0, e1] = [c0, c1].map(|c| {
            let len = seen.len();
            *seen.entry(c).or_insert_with(|| {
                graph.push(BTreeSet::new());
                len
            })
        });
        graph[e0].insert(e1);
        graph[e1].insert(e0);
    }
    // maybe just bool if contains t?
    let translation = seen
        .into_iter()
        .sorted_by_key(|(_k, v)| *v)
        .map(|(k, _)| k)
        .collect();
    (graph, translation)
}

fn solve() {
    let (g, trans) = parse();
    let nodes = g.len();
    // first get all cliques of size 3
    let mut cliques = BTreeSet::new();
    for node in 0..g.len() {
        for (one, two) in g[node].iter().tuple_combinations() {
            if g[*one].contains(two) && g[*two].contains(one) {
                let mut clique = vec![node, *one, *two];
                clique.sort_unstable();
                cliques.insert(clique);
            }
        }
    }
    let c = cliques.iter().filter(|f| f.iter().any(|x| trans[*x].starts_with('t'))).count();
    println!("{c}");

    loop {
        println!("{} cliques of size {}", cliques.len(), cliques.first().unwrap().len());
        if cliques.len() == 1 {
            break;
        }
        let mut bigger_cliques = BTreeSet::new();
        for clique in &cliques {
            // do intersection of all members
            let mut candidates: BTreeSet<usize> = BTreeSet::from_iter(0..nodes);
            for member in clique {
                candidates.retain(|c| g[*member].contains(c));
            }
            for candidate in candidates {
                let mut clique = clique.clone();
                let pos = clique.binary_search(&candidate).unwrap_or_else(|e| e);
                clique.insert(pos, candidate);
                bigger_cliques.insert(clique);
            }
        }
        cliques = bigger_cliques;

    }
    let answer = cliques.first().unwrap();
    let s = answer.iter().map(|e| &trans[*e]).sorted_unstable().join(",");
    println!("{s}");

}
