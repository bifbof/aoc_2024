use std::iter::zip;

fn main() {
    // for part 1 you only have to change Operator to remove concat.
    part();
}

#[derive(Clone, Debug, Copy, Default)]
enum Operator {
    #[default]
    Concat,
    Mul,
    Add,
}

impl Operator {
    fn apply(self, lhs: usize, rhs: usize) -> usize {
        match self {
            Self::Concat => lhs * (10_usize.pow(rhs.ilog10() + 1)) + rhs,
            Self::Mul => lhs * rhs,
            Self::Add => lhs + rhs,
        }
    }

    fn next_overflowing(self) -> (Self, bool) {
        match self {
            Self::Concat => (Self::Mul, false),
            Self::Mul => (Self::Add, false),
            Self::Add => (Self::default(), true),
        }
    }
}

fn next_decision(mut decision: Vec<Operator>) -> Option<Vec<Operator>> {
    for d in decision.iter_mut().rev() {
        let (next, overflow) = d.next_overflowing();
        *d = next;
        if !overflow {
            return Some(decision);
        }
    }
    None
}

#[derive(Debug)]
struct Numbers {
    decision: Option<Vec<Operator>>,
    nums: Vec<usize>,
}

impl Numbers {
    fn create(nums: Vec<usize>) -> Self {
        let decision = Some((0..nums.len()).map(|_| Operator::default()).collect());
        // potential optimization move operator of first entry directly to Operator::Add
        // -> 3x less work
        Self { decision, nums }
    }
}

impl Iterator for Numbers {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let decision = self.decision.clone()?;
        let b = self
            .nums
            .iter()
            .enumerate()
            .fold(0, |acc: usize, (idx, &val)| decision[idx].apply(acc, val));
        self.decision = next_decision(decision);
        Some(b)
    }
}

fn parse() -> (Vec<usize>, Vec<Vec<usize>>) {
    let data = std::fs::read_to_string("input/07.txt").expect("no file :/");
    let mut target = Vec::new();
    let mut numbers = Vec::new();

    for line in data.lines() {
        let (t, line) = line.split_once(':').unwrap();
        target.push(t.parse().unwrap());
        let line = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        numbers.push(line);
    }

    (target, numbers)
}

fn part() {
    let (target, numbers) = parse();
    let mut count = 0;
    for (t, n) in zip(target, numbers) {
        let n = Numbers::create(n);
        for i in n {
            if i == t {
                count += t;
                break;
            }
        }
    }
    println!("{count}");
}
