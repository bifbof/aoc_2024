use itertools::Itertools;

type Grid = Vec<Vec<char>>;
type Moves = Vec<Move>;

#[derive(Debug)]
enum Move {
    Up,
    Right,
    Down,
    Left,
}

impl Move {
    fn step(&self, pos: (usize, usize)) -> (usize, usize) {
        let (x, y) = pos;
        match self {
            Self::Up => (x - 1, y),
            Self::Right => (x, y + 1),
            Self::Down => (x + 1, y),
            &Self::Left => (x, y - 1),
        }
    }
}

fn main() {
    solve(true);
    solve(false);
}

fn parse_moves(moves: &str) -> Moves {
    moves
        .lines()
        .flat_map(|line| line.chars())
        .map(|c| match c {
            '^' => Move::Up,
            '>' => Move::Right,
            'v' => Move::Down,
            '<' => Move::Left,
            _ => unreachable!("invalid move"),
        })
        .collect()
}

fn parse1() -> (Grid, Moves) {
    let data = std::fs::read_to_string("input/15.txt").unwrap();
    let (grid, moves) = data.split_once("\n\n").unwrap();
    let grid = grid.lines().map(|t| t.chars().collect()).collect();
    (grid, parse_moves(moves))
}

fn parse2() -> (Grid, Moves) {
    let data = std::fs::read_to_string("input/15.txt").unwrap();
    let (grid, moves) = data.split_once("\n\n").unwrap();
    let mut out = Vec::new();
    for line in grid.lines() {
        let line = line
            .chars()
            .flat_map(|c| match c {
                '#' => ['#', '#'],
                '.' => ['.', '.'],
                'O' => ['[', ']'],
                '@' => ['@', '.'],
                _ => unreachable!(),
            })
            .collect();
        out.push(line);
    }
    (out, parse_moves(moves))
}

fn can_move(grid: &Grid, pos: (usize, usize), m: &Move, rec: bool) -> bool {
    match grid[pos.0][pos.1] {
        '#' => false,
        '.' => true,
        '@' | 'O' => can_move(grid, m.step(pos), m, true),
        val @ ('[' | ']') => {
            let mut b = can_move(grid, m.step(pos), m, true);
            if matches!(m, Move::Up | Move::Down) {
                b = b
                    && match (rec, val) {
                        (true, '[') => can_move(grid, Move::Right.step(pos), m, false),
                        (true, ']') => can_move(grid, Move::Left.step(pos), m, false),
                        _ => true,
                    };
            }
            b
        }
        _ => unreachable!(),
    }
}

fn do_move(grid: &mut Grid, pos: (usize, usize), m: &Move) {
    match grid[pos.0][pos.1] {
        '#' | '.' => {}
        val @ ('@' | 'O' | '[' | ']') => {
            do_move(grid, m.step(pos), m);
            let neigh = m.step(pos);
            grid[neigh.0][neigh.1] = val;
            grid[pos.0][pos.1] = '.';
            match (val, m) {
                ('[', Move::Up | Move::Down) => do_move(grid, Move::Right.step(pos), m),
                (']', Move::Up | Move::Down) => do_move(grid, Move::Left.step(pos), m),
                _ => {}
            }
        }
        _ => unreachable!(),
    }
}

fn solve(part1: bool) {
    let (mut grid, moves) = if part1 { parse1() } else { parse2() };
    let nrows = grid.len();
    let ncols = grid.first().unwrap().len();
    let mut pos = (1..nrows)
        .cartesian_product(1..ncols)
        .find(|c| grid[c.0][c.1] == '@')
        .unwrap();
    for m in moves {
        if can_move(&grid, pos, &m, true) {
            do_move(&mut grid, pos, &m);
            pos = m.step(pos);
        }
    }

    let mut count = 0;
    for (ridx, row) in grid.iter().enumerate() {
        for (cidx, val) in row.iter().enumerate() {
            count += match (val, part1) {
                (&'O', true) | (&'[', false) => ridx * 100 + cidx,
                _ => 0,
            };
        }
    }
    println!("{count}");
}
