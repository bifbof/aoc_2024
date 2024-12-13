use itertools::Itertools;

fn main() {
    part1();
    part2();
}

fn parse() -> Vec<Vec<char>> {
    let data = std::fs::read_to_string("input/08.txt").expect("cannot read file");
    data.lines().map(|line| line.chars().collect()).collect()
}

fn add_vector(
    p: &(usize, usize),
    delta: &(isize, isize),
    borders: &(usize, usize),
) -> Option<(usize, usize)> {
    let x = p.0.checked_add_signed(delta.0)?;
    let y = p.1.checked_add_signed(delta.1)?;
    if x < borders.0 && y < borders.1 {
        Some((x, y))
    } else {
        None
    }
}

fn part1() {
    let grid = parse();
    let nrows = grid.len();
    let ncols = grid.first().unwrap().len();
    let mut antinodes = vec![vec![false; ncols]; nrows];
    let symbols: Vec<char> = (0..nrows)
        .cartesian_product(0..ncols)
        .map(|(r, c)| grid[r][c])
        .filter(|&v| v != '.')
        .unique()
        .collect();
    for symbol in symbols {
        let occurences: Vec<(usize, usize)> = (0..nrows)
            .cartesian_product(0..ncols)
            .filter(|(r, c)| grid[*r][*c] == symbol)
            .collect();

        for (p0, p1) in occurences.iter().cartesian_product(&occurences) {
            if p0 == p1 {
                continue;
            }
            let delta = (p0.0 as isize - p1.0 as isize, p0.1 as isize - p1.1 as isize);
            // println!("p0 {:?}, p1 {:?}, delta {:?}", p0, p1, delta);
            if let Some(pos) = add_vector(p0, &delta, &(nrows, ncols)) {
                if !antinodes[pos.0][pos.1] {
                    antinodes[pos.0][pos.1] = true;
                }
            }
            if let Some(pos) = add_vector(p1, &(-delta.0, -delta.1), &(nrows, ncols)) {
                if !antinodes[pos.0][pos.1] {
                    antinodes[pos.0][pos.1] = true;
                }
            }
        }
    }
    let c = (0..nrows)
        .cartesian_product(0..ncols)
        .filter(|(r, c)| antinodes[*r][*c]).count();
    println!("{c}");
    // println!("{:?}", grid);
}

fn part2() {
    let grid = parse();
    let nrows = grid.len();
    let ncols = grid.first().unwrap().len();
    let mut antinodes = vec![vec![false; ncols]; nrows];
    let symbols: Vec<char> = (0..nrows)
        .cartesian_product(0..ncols)
        .map(|(r, c)| grid[r][c])
        .filter(|&v| v != '.')
        .unique()
        .collect();
    for symbol in symbols {
        let occurences: Vec<(usize, usize)> = (0..nrows)
            .cartesian_product(0..ncols)
            .filter(|(r, c)| grid[*r][*c] == symbol)
            .collect();

        for (p0, p1) in occurences.iter().cartesian_product(&occurences) {
            let mut p0 = *p0;
            let mut p1 = *p1;
            if p0 == p1 {
                continue;
            }
            let delta = (p0.0 as isize - p1.0 as isize, p0.1 as isize - p1.1 as isize);
            while let Some(p) = add_vector(&p0, &delta, &(nrows, ncols))  {
                p0 = p;
                if !antinodes[p.0][p.1] {
                    antinodes[p.0][p.1] = true;
                }
            }
            // I made some mistake here, but funnily it is the right mistake ...
            // I don't understand it xD
            while let Some(p) = add_vector(&p1, &delta, &(nrows, ncols)) {
                p1 = p;
                if !antinodes[p.0][p.1] {
                    antinodes[p.0][p.1] = true;
                }
            }
        }
    }
    let c = (0..nrows)
        .cartesian_product(0..ncols)
        .filter(|(r, c)| antinodes[*r][*c]).count();
    println!("{c}");
}
