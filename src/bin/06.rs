/**Todays implementation is ugly, but took me long-enough
 * Possible optimizations:
 * - look only at path of guard -> interruption must happen there
 * - This Guard-step datastructure-code does not really work nicely.
 */
use std::ops::Rem;

type Grid = Vec<Vec<bool>>;

#[derive(Clone, Debug)]
struct Guard {
    pos: (usize, usize),
    dir: (isize, isize),
    dir_id: usize,
}

impl Guard {
    const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    fn create(pos: (usize, usize)) -> Self {
        Self {
            pos,
            dir: (-1, 0),
            dir_id: 0,
        }
    }

    fn step(&self, borders: (usize, usize)) -> Option<Self> {
        let pos = (
            self.pos.0.checked_add_signed(self.dir.0)?,
            self.pos.1.checked_add_signed(self.dir.1)?,
        );
        if pos.0 < borders.0 && pos.1 < borders.1 {
            Some(Self {
                pos,
                dir: self.dir,
                dir_id: self.dir_id,
            })
        } else {
            None
        }
    }

    fn rotate90(&self) -> Self {
        let dir_id = (self.dir_id + 1).rem(Self::DIRS.len());
        let dir = *Self::DIRS.get(dir_id).unwrap();
        Self {
            pos: self.pos,
            dir,
            dir_id,
        }
    }
}

fn main() {
    part1();
    part2();
}

fn parse() -> (Grid, Guard) {
    let data = std::fs::read_to_string("input/06.txt").expect("cannot read file");
    let mut grid = Grid::new();
    let mut position = Guard::create((usize::MAX, usize::MAX));
    for (row, line) in data.lines().enumerate() {
        let line = line
            .chars()
            .enumerate()
            .map(|(col, ch)| match ch {
                '#' => true,
                '^' => {
                    position = Guard::create((row, col));
                    false
                }
                _ => false,
            })
            .collect();
        grid.push(line);
    }
    (grid, position)
}

fn part1() {
    let (grid, mut guard) = parse();
    let nrows = grid.len();
    let ncols = grid.first().unwrap().len();
    let mut path: Grid = (0..nrows)
        .map(|_| (0..ncols).map(|_| false).collect())
        .collect();

    loop {
        path[guard.pos.0][guard.pos.1] = true;
        let new_guard = match guard.step((nrows, ncols)) {
            None => break, // stepped outsize grid -> finished
            Some(pos) => pos,
        };
        match grid[new_guard.pos.0][new_guard.pos.1] {
            true => guard = guard.rotate90(),
            false => guard = new_guard,
        }
    }
    let c: usize = path
        .iter()
        .map(|row| row.iter().map(|&b| usize::from(b)).sum::<usize>())
        .sum();
    println!("{c}");
}

fn part2() {
    let (grid, start_guard) = parse();
    let nrows = grid.len();
    let ncols = grid.first().unwrap().len();

    let mut count = 0;
    for row in 0..nrows {
        for col in 0..ncols {
            // invalid positions for a stone
            if (row, col) == start_guard.pos || grid[row][col] {
                continue;
            }
            let mut guard = start_guard.clone();
            let mut path: Vec<Vec<[bool; 4]>> = (0..nrows)
                .map(|_| (0..ncols).map(|_| [false; 4]).collect())
                .collect();

            let mut grid = grid.clone();
            grid[row][col] = true;
            let grid = grid;

            loop {
                if path[guard.pos.0][guard.pos.1][guard.dir_id] {
                    count += 1;
                    break;
                }
                path[guard.pos.0][guard.pos.1][guard.dir_id] = true;
                let new_guard = match guard.step((nrows, ncols)) {
                    None => break, // stepped outsize grid -> no loop
                    Some(g) => g,
                };
                match grid[new_guard.pos.0][new_guard.pos.1] {
                    true => guard = guard.rotate90(),
                    false => guard = new_guard,
                }
            }
        }
    }
    println!("{count}");
}
