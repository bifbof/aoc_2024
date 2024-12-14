// I never want to look at this code again.
// what a overcomplicated piece.
// I think the directions would be nice, but for such a small
// exercise overkill.

type Grid = Vec<Vec<char>>;
type Pos = (usize, usize);
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    part1();
    part2();
}

fn parse() -> Grid {
    let data = std::fs::read_to_string("input/12.txt").unwrap();
    data.lines().map(|line| line.chars().collect()).collect()
}

fn part1() {
    let grid = parse();
    let nrows = grid.len();
    let ncols = grid.first().unwrap().len();
    let mut visited = vec![vec![false; ncols]; nrows];

    let mut count = 0;
    for (x, y) in (0..nrows).cartesian_product(0..ncols) {
        if visited[x][y] {
            continue;
        }
        let value = grid[x][y];
        let mut area = 0;
        let mut peri = 0;
        let mut queue = VecDeque::from([Some((x, y))]);
        while let Some(pos) = queue.pop_front() {
            let (x, y) = match pos {
                None => {
                    peri += 1;
                    continue;
                }
                Some(pos) => pos,
            };
            if grid[x][y] != value {
                peri += 1;
                continue;
            }
            if visited[x][y] {
                continue;
            }
            visited[x][y] = true;
            area += 1;
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let val = match (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                    (Some(x), Some(y)) if x < nrows && y < ncols => Some((x, y)),
                    _ => None,
                };
                queue.push_back(val);
            }
        }
        // println!("{value}: {} * {} = {}", area, peri, area * peri);
        count += area * peri;
    }
    println!("{count}");
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

struct Neighbor {
    pos: Option<Pos>,
    edge: [Pos; 2],
    dir: Direction,
}

impl Neighbor {
    fn in_direction_of(pos: Pos, dir: Direction, grid: &Grid) -> Neighbor {
        let (x, y) = pos;
        let nrows = grid.len();
        let ncols = grid.first().unwrap().len();
        let (dx, dy) = match dir {
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0),
        };
        let pos = match (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
            (Some(x), Some(y)) if x < nrows && y < ncols => Some((x, y)),
            _ => None,
        };
        let edge = match dir {
            Direction::Right => [(x, y + 1), (x + 1, y + 1)],
            Direction::Down => [(x + 1, y + 1), (x + 1, y)],
            Direction::Left => [(x + 1, y), (x, y)],
            Direction::Up => [(x, y), (x, y + 1)],
        };
        Neighbor { pos, edge, dir }
    }
}

struct Point<'grid> {
    pos: Pos,
    grid: &'grid Grid,
    dir: Option<Direction>,
}
impl<'grid> Point<'grid> {
    fn new(pos: Pos, grid: &'grid Grid) -> Point<'grid> {
        Point {
            pos,
            grid,
            dir: None,
        }
    }
}

impl Iterator for Point<'_> {
    type Item = Neighbor;
    fn next(&mut self) -> Option<Self::Item> {
        let dir = match self.dir {
            None => Direction::Right,
            Some(Direction::Right) => Direction::Down,
            Some(Direction::Down) => Direction::Left,
            Some(Direction::Left) => Direction::Up,
            Some(Direction::Up) => return None,
        };
        self.dir = Some(dir);
        Some(Neighbor::in_direction_of(self.pos, dir, self.grid))
    }
}

fn part2() {
    let grid = parse();
    let nrows = grid.len();
    let ncols = grid.first().unwrap().len();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; ncols]; nrows];
    let mut count = 0;

    for (x, y) in (0..nrows).cartesian_product(0..ncols) {
        if visited[x][y] {
            continue;
        }
        let value = grid[x][y];
        let mut area = 0;
        let mut queue = VecDeque::from([(x, y)]);
        let mut edges: HashMap<Pos, HashSet<Direction>> = HashMap::new();

        while let Some((x, y)) = queue.pop_back() {
            if visited[x][y] {
                continue;
            }
            visited[x][y] = true;
            area += 1;
            let pnt = Point::new((x, y), &grid);
            for neigh in pnt {
                match neigh.pos {
                    Some((x, y)) if grid[x][y] == value => queue.push_back((x, y)),
                    _ => {
                        edges
                            .entry(neigh.edge[0])
                            .or_default()
                            .insert(neigh.dir);
                        edges
                            .entry(neigh.edge[1])
                            .or_default()
                            .insert(neigh.dir);
                    }
                }
            }
        }
        let mut perimeter = 0;
        for val in edges.values() {
            perimeter += val.len() / 2;
        }
        count += perimeter * area;
    }
    println!("{count}");
}
