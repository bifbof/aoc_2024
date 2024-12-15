use std::fs;

fn main() {
    part1();
    part2();
}

fn parse() -> Vec<Vec<char>> {
    let file = "input/04.txt";
    let data = fs::read_to_string(file).expect("couldn't read the file");
    let mut width = None;
    let mut out = Vec::new();
    for line in data.lines() {
        let chars: Vec<char> = line.chars().collect();
        match width {
            None => width = Some(chars.len()),
            Some(v) => assert!(chars.len() == v),
        }
        out.push(chars);
    }
    out
}

const DIRS8: [[isize; 2]; 8] = [
    [0, 1],
    [1, 1],
    [1, 0],
    [1, -1],
    [0, -1],
    [-1, -1],
    [-1, 0],
    [-1, 1],
];

fn part1() {
    let chars = parse();
    let rows = chars.len();
    let cols = chars.first().unwrap().len();
    let mut count_xmas = 0;
    for r in 0..rows {
        for c in 0..cols {
            if chars[r][c] != 'X' {
                continue;
            }
            // check into all 8 directions
            for d in DIRS8 {
                let mut xpos = Some(r);
                let mut ypos = Some(c);
                let mut found = true;
                for letter in "MAS".chars() {
                    xpos = match xpos.map(|v| v.checked_add_signed(d[0])) {
                        Some(Some(x)) if x < rows => Some(x),
                        _ => None,
                    };
                    ypos = match ypos.map(|v| v.checked_add_signed(d[1])) {
                        Some(Some(x)) if x < cols => Some(x),
                        _ => None,
                    };
                    let Some(xpos) = xpos else {
                        found = false;
                        break;
                    };
                    let Some(ypos) = ypos else {
                        found = false;
                        break;
                    };
                    found = found && (chars[xpos][ypos] == letter);
                }
                if found {
                    count_xmas += 1;
                }
            }
        }
    }
    println!("num xmas {count_xmas}");
}

const DIRS4: [[isize; 2]; 4] = [[1, 1], [1, -1], [-1, -1], [-1, 1]];

fn part2() {
    let chars = parse();
    let rows = chars.len();
    let cols = chars.first().unwrap().len();
    let mut count_xmas = 0;
    for r in 0..rows {
        for c in 0..cols {
            if chars[r][c] != 'A' {
                continue;
            }
            // check into all 4 directions
            let mut corners = ['X'; 4];
            let mut num_m = 0;
            let mut num_s = 0;
            for (idx, d) in DIRS4.iter().enumerate() {
                let r = match r.checked_add_signed(d[0]) {
                    Some(x) if x < rows => x,
                    _ => break,
                };
                let c = match c.checked_add_signed(d[1]) {
                    Some(x) if x < cols => x,
                    _ => break,
                };
                match chars[r][c] {
                    'M' => num_m += 1,
                    'S' => num_s += 1,
                    _ => {}
                }
                corners[idx] = chars[r][c];
            }
            let not_mam = corners[1] != corners[3];
            if num_m == 2 && num_s == 2 && not_mam {
                count_xmas += 1;
            }
        }
    }
    println!("num x-mas {count_xmas}");
}
