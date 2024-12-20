use itertools::Itertools;
use std::{collections::VecDeque, ops::Deref};

type Start = (usize, usize);
type Goal = (usize, usize);

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

impl<T> Grid<T> {
    fn checked_add_signed(
        &self,
        pos: (usize, usize),
        delta: (isize, isize),
    ) -> Option<(usize, usize)> {
        let (r, c) = pos;
        let (dr, dc) = delta;
        match (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
            (Some(r), Some(c)) if r < self.nrows && c < self.ncols => Some((r, c)),
            _ => None,
        }
    }
}

fn main() {
    solve(2, 100);
    solve(20, 100);
}

fn parse() -> (Grid<char>, Start, Goal) {
    let data = std::fs::read_to_string("input/20.txt").unwrap();
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

fn shortest_path(grid: &Grid<char>, start: (usize, usize)) -> Vec<Vec<usize>> {
    let mut dist: Vec<Vec<usize>> = (0..grid.nrows)
        .map(|_| (0..grid.ncols).map(|_| usize::MAX).collect())
        .collect();
    dist[start.0][start.1] = 0;
    let mut queue = VecDeque::from([start]);

    while let Some((r, c)) = queue.pop_front() {
        if grid[r][c] == '#' {
            continue;
        }

        for delta in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let Some((nr, nc)) = grid.checked_add_signed((r, c), delta) else {
                continue;
            };
            if grid[nr][nc] == '#' {
                continue;
            }
            let cost = dist[r][c] + 1;
            if cost < dist[nr][nc] {
                queue.push_back((nr, nc));
                dist[nr][nc] = cost;
            }
        }
    }
    dist
}

fn solve(ncheat: isize, saved: usize) {
    let (grid, start, goal) = parse();
    let dist_start = shortest_path(&grid, start);
    let dist_goal = shortest_path(&grid, goal);

    let threshold = dist_start[goal.0][goal.1].saturating_sub(saved);

    let mut nskips = 0;
    for (r, c) in (0..grid.nrows).cartesian_product(0..grid.ncols) {
        if dist_start[r][c] == usize::MAX {
            continue;
        }
        for (dr, dc) in (-ncheat..=ncheat).cartesian_product(-ncheat..=ncheat) {
            let Some((nr, nc)) = grid.checked_add_signed((r, c), (dr, dc)) else {
                continue;
            };
            if dist_start[nr][nc] == usize::MAX {
                continue;
            }
            let dbetween = r.abs_diff(nr) + c.abs_diff(nc);
            if dbetween > ncheat.try_into().unwrap() {
                continue;
            }
            let distance = dist_start[r][c]
                .saturating_add(dbetween)
                .saturating_add(dist_goal[nr][nc]);
            if distance <= threshold {
                nskips += 1;
            }
        }
    }
    println!("{nskips}");
}
