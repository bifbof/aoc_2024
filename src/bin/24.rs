use std::collections::HashMap;

enum Op {
    Xor,
    And,
    Or,
}
enum Entry {
    Value(bool),
    Gate { a: String, b: String, op: Op },
}

fn main() {
    let g = parse();
    let mut num = 0;
    for i in 0..46 {
        let z = format!("z{i:02}");
        let val = u64::from(dfs(&z, &g));
        num |= val << i;
    }
    println!("{num}");
    // part 2 done by hand
}

fn dfs(node: &String, graph: &HashMap<String, Entry>) -> bool {
    match &graph[node] {
        Entry::Value(b) => *b,
        Entry::Gate { a, b, op } => {
            let a = dfs(a, graph);
            let b = dfs(b, graph);
            match op {
                Op::Xor => a ^ b,
                Op::And => a & b,
                Op::Or => a | b,
            }
        }
    }
}

fn parse() -> HashMap<String, Entry> {
    let data = std::fs::read_to_string("input/24.txt").unwrap();
    let (nums, edges) = data.split_once("\n\n").unwrap();
    let mut graph: HashMap<_, _> = nums
        .lines()
        .map(|t| t.split_once(": ").unwrap())
        .map(|(s, v)| (s.to_string(), Entry::Value(v == "1")))
        .collect();
    for line in edges.lines() {
        let (to, from) = line.split_once(" -> ").unwrap();
        let mut to = to.split_whitespace();
        let (Some(a), Some(op), Some(b), None) = (to.next(), to.next(), to.next(), to.next())
        else {
            panic!();
        };
        let op = match op {
            "XOR" => Op::Xor,
            "AND" => Op::And,
            "OR" => Op::Or,
            _ => unreachable!(),
        };
        graph.insert(
            from.to_string(),
            Entry::Gate {
                a: a.to_string(),
                b: b.to_string(),
                op,
            },
        );
    }
    graph
}
