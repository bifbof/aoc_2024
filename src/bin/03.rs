use std::fs;

struct Mul {
    left: usize,
    right: usize,
}

fn main() {
    let data = fs::read_to_string("input/03.txt").expect("Unable to read file");
    let mut muls: Vec<Mul> = Vec::new();
    let mut idx = 0;
    /* part 2 */
    let mut state = true;
    /* part 2 */

    loop {
        /* part 2 */
        match data.get(idx..idx + 7) {
            None => break,
            Some("don't()") => {
                state = false;
                idx += 7;
            },
            Some(_) => {},
        }
        match data.get(idx..idx+4) {
            None => break,
            Some("do()") => {
                state = true;
                idx += 4;
            },
            Some(_) => {},
        }
        if !state {
            idx += 1;
            continue;
        }
        /* part 2 */

        // try to parse mul
        match data.get(idx..idx + 4) {
            None => break,
            Some("mul(") => idx += 4,
            Some(_) => {
                idx += 1;
                continue;
            }
        }

        // left number
        let offset = match parse_number(data.get(idx..)) {
            None => break,
            Some(0) => continue,
            Some(v) => v,
        };
        let left: usize = data.get(idx..idx+offset).unwrap().parse().unwrap();
        idx += offset;

        // ,
        match data.get(idx..idx + 1) {
            None => break,
            Some(",") => idx += 1,
            Some(_) => continue,
        }

        // right number
        let offset = match parse_number(data.get(idx..)) {
            None => break,
            Some(0) => continue,
            Some(v) => v,
        };
        let right: usize = data.get(idx..idx+offset).unwrap().parse().unwrap();
        idx += offset;

        // )
        match data.get(idx..idx + 1) {
            None => break,
            Some(")") => idx += 1,
            Some(_) => continue,
        }
        muls.push(Mul { left, right });
    }
    let mut sum: usize = 0;
    for mul in muls {
        sum += mul.left * mul.right;
    }
    println!("{sum}");
}

fn parse_number(s: Option<&str>) -> Option<usize> {
    let s = s?;
    let mut idx = 0;
    loop {
        match s.get(idx..=idx) {
            None => return None,
            Some("0" | "1" | "2" | "3" | "4" | "5"| "6" | "7" | "8" | "9") => idx += 1,
            Some(_) => break,
        }
    }
    Some(idx)
}
