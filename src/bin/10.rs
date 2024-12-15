use std::collections::HashSet;

// simpler solution would use graph algorithm. :)

fn main() {
    part1();
    part2();
}

#[allow(clippy::type_complexity)]
fn parse() -> (Vec<Vec<usize>>, [Vec<(usize, usize)>; 10]) {
    let data = std::fs::read_to_string("input/10.txt").expect("msg");
    let mut out = Vec::new();
    let mut indices = [(); 10].map(|_| Vec::new());
    for (row, line) in data.lines().enumerate() {
        let mut rowv = Vec::new();
        for (col, c) in line.chars().enumerate() {
            let value = usize::try_from(c.to_digit(10).unwrap()).unwrap();
            rowv.push(value);
            indices[value].push((row, col));
        }
        out.push(rowv);
    }
    (out, indices)
}

struct Neighbors {
    x: usize,
    y: usize,
    nrows: usize,
    ncols: usize,
    state: usize,
}

impl Neighbors {
    fn create(pos: (usize, usize), borders: (usize, usize)) -> Self {
        Self {
            x: pos.0,
            y: pos.1,
            nrows: borders.0,
            ncols: borders.1,
            state: 0,
        }
    }
}

impl Iterator for Neighbors {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.state < 4 {
            let (dx, dy) = [(0, 1), (1, 0), (0, -1), (-1, 0)][self.state];
            self.state += 1;
            let x = self.x.checked_add_signed(dx);
            let y = self.y.checked_add_signed(dy);
            return match (x, y) {
                (Some(x), Some(y)) if x < self.nrows && y < self.ncols => Some((x, y)),
                _ => continue,
            }
        }
        None
    }
}

fn part1() {
    let (data, idx) = parse();
    let nrows = data.len();
    let ncols = data.first().unwrap().len();
    let mut counter = vec![vec![HashSet::new(); ncols]; nrows];

    // initialize
    for (x, y) in idx[9].iter() {
        counter[*x][*y].insert((*x, *y));
    }

    // calculate updates
    for val in (0..9).rev() {
        for (x, y) in idx[val].iter() {
            let neighbors = Neighbors::create((*x, *y), (nrows, ncols));
            for (nx, ny) in neighbors {
                if data[*x][*y] + 1 == data[nx][ny] {
                    let union: HashSet<_> = counter[*x][*y].union(&counter[nx][ny]).copied().collect();
                    counter[*x][*y] = union;
                }
            }
        }
    }
    // extract solution
    let sum: usize = idx[0].iter().map(|(x, y)| counter[*x][*y].len()).sum();
    println!("{}", sum);
}

// well well well if part 1 accidently solved part 2 and I had to repair it.
fn part2() {
    let (data, idx) = parse();
    let nrows = data.len();
    let ncols = data.first().unwrap().len();
    let mut counter = vec![vec![0; ncols]; nrows];

    // initialize
    for (x, y) in &idx[9] {
        counter[*x][*y] = 1;
    }

    // calculate updates
    for val in (0..9).rev() {
        for (x, y) in idx[val].iter() {
            let neighbors = Neighbors::create((*x, *y), (nrows, ncols));
            for (nx, ny) in neighbors {
                if data[*x][*y] + 1 == data[nx][ny] {
                    counter[*x][*y] += counter[nx][ny];
                }
            }
        }
    }
    // extract solution
    let sum: usize = idx[0].iter().map(|(x, y)| counter[*x][*y]).sum();
    println!("{}", sum);
}
