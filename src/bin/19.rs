fn main() {
    solve();
}

fn parse() -> (Vec<String>, Vec<String>) {
    let data = std::fs::read_to_string("input/19.txt").unwrap();
    let (designs, patterns) = data.split_once("\n\n").unwrap();
    let patterns = patterns.lines().map(|p| p.to_string()).collect();
    let designs: Vec<String> = designs.split(", ").map(|d| d.to_string()).collect();
    (designs, patterns)
}

fn solve() {
    let (designs, patterns) = parse();
    let mut p1 = 0;
    let mut p2 = 0;
    for pattern in patterns {
        let num_possible = possible_pattern(&pattern, &designs);
        if num_possible > 1 {
            p1 += 1;
        }
        p2 += num_possible;
    }
    println!("{p1}");
    println!("{p2}");
}

fn possible_pattern(pattern: &str, designs: &Vec<String>) -> usize {
    let mut possible = vec![0_usize; pattern.len() + 1];
    *possible.last_mut().unwrap() = 1;

    for idx in (0..pattern.len()).rev() {
        for design in designs {
            if pattern[idx..].starts_with(design) {
                possible[idx] += possible[idx + design.len()];
            }
        }
    }
    *possible.first().unwrap()
}
