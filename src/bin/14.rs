use itertools::Itertools;
use std::{cmp::Ordering, ops::Rem};

type Pos = (usize, usize);
type Vel = (isize, isize);

fn main() {
    part1(103, 101, 100);
    part2(103, 101);
}

fn parse() -> (Vec<Pos>, Vec<Vel>) {
    let data = std::fs::read_to_string("input/14.txt").unwrap();
    let mut positions = Vec::new();
    let mut velocities = Vec::new();
    for line in data.lines() {
        let (pos, vel) = line.split_once(' ').unwrap();
        // get rid of p=
        let (pos, vel) = (&pos[2..], &vel[2..]);
        let (pos_col, pos_row) = pos.split_once(',').unwrap();
        let (vel_col, vel_row) = vel.split_once(',').unwrap();
        let pos_col = pos_col.parse().unwrap();
        let pos_row = pos_row.parse().unwrap();
        let vel_col = vel_col.parse().unwrap();
        let vel_row = vel_row.parse().unwrap();
        positions.push((pos_row, pos_col));
        velocities.push((vel_row, vel_col));
    }
    (positions, velocities)
}

fn part1(nrows: usize, ncols: usize, nsecs: isize) {
    let (robot_pos, robot_vel) = parse();
    // let grid = vec![vec![0_usize; ncols]; nrows];
    let rmid = nrows / 2;
    let cmid = ncols / 2;

    let mut q = [0; 4];
    for ((x, y), (vx, vy)) in robot_pos.iter().zip(robot_vel) {
        // if we push velocity directly into usize, multiple conversions
        // wouldn't be necessary anymore. :)
        let x = x
            .wrapping_add_signed((nsecs * vx).rem_euclid(nrows as isize))
            .rem(nrows);
        let y = y
            .wrapping_add_signed((nsecs * vy).rem_euclid(ncols as isize))
            .rem(ncols);
        let idx = match x.cmp(&rmid) {
            Ordering::Less => 0,
            Ordering::Equal => continue,
            Ordering::Greater => 2,
        };
        let idx = match y.cmp(&cmid) {
            Ordering::Less => idx,
            Ordering::Equal => continue,
            Ordering::Greater => idx + 1,
        };
        q[idx] += 1;
    }
    println!("{}", q.iter().product::<usize>());
}

fn print_grid(grid: &Vec<Vec<usize>>) {
    print!("\x1B[2J");
    for row in grid {
        for val in row {
            if *val == 0 {
                print!(".");
            } else {
                print!("*");
            }
        }
        println!();
    }
}

fn dfs_preparation(grid: &Vec<Vec<usize>>, nrows: usize, ncols: usize) -> usize {
    let mut visited = vec![vec![false; ncols]; nrows];
    let mut max = 0;
    for (x, y) in (0..nrows).cartesian_product(0..ncols) {
        let val = dfs(x, y, grid, &mut visited, nrows, ncols);
        max = max.max(val);
    }
    max
}

fn dfs(
    x: usize,
    y: usize,
    grid: &Vec<Vec<usize>>,
    visited: &mut Vec<Vec<bool>>,
    nrows: usize,
    ncols: usize,
) -> usize {
    if visited[x][y] {
        return 0;
    }
    visited[x][y] = true;
    if grid[x][y] == 0 {
        return 0;
    }
    let mut sum = 0;
    for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
        sum += match (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
            (Some(x), Some(y)) if x < nrows && y < ncols => dfs(x, y, grid, visited, nrows, ncols),
            _ => 0,
        }
    }
    1 + sum
}

fn part2(nrows: usize, ncols: usize) {
    let mut niter = 0;
    let (mut pos, vel) = parse();
    loop {
        let mut grid = vec![vec![0_usize; ncols]; nrows];
        for ((x, y), (vx, vy)) in pos.iter_mut().zip(&vel) {
            *x = x
                .wrapping_add_signed(vx.rem_euclid(nrows as isize))
                .rem(nrows);
            *y = y
                .wrapping_add_signed(vy.rem_euclid(ncols as isize))
                .rem(ncols);
            grid[*x][*y] += 1;
        }
        let longest_line = dfs_preparation(&grid, nrows, ncols);
        println!("{longest_line}");
        niter += 1;
        if longest_line > 100 {
            print_grid(&grid);
            println!("{niter}");
            let mut s = String::new();
            std::io::stdin()
                .read_line(&mut s)
                .expect("Did not enter a correct string");
        }
    }
}
