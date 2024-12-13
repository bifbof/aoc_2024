fn main() {
    solve();
}

fn parse() -> Vec<[(isize, isize); 3]> {
    let data = std::fs::read_to_string("input/13.txt").unwrap();
    let mut games = Vec::new();
    for game in data.split("\n\n") {
        let mut game = game.lines();
        let (Some(a), Some(b), Some(price)) = (game.next(), game.next(), game.next()) else {
            panic!()
        };
        let mut a = a.split('+');
        let (_, Some(ax), Some(ay)) = (a.next(), a.next(), a.next()) else {
            panic!()
        };
        let (ax, _) = ax.rsplit_once(',').unwrap();

        let mut b = b.split('+');
        let (_, Some(bx), Some(by)) = (b.next(), b.next(), b.next()) else {
            panic!()
        };
        let (bx, _) = bx.rsplit_once(',').unwrap();

        let mut price = price.split('=');
        let (_, Some(pricex), Some(pricey)) = (price.next(), price.next(), price.next()) else {
            panic!()
        };
        let (pricex, _) = pricex.split_once(',').unwrap();
        let ax = ax.parse().unwrap();
        let ay = ay.parse().unwrap();
        let bx = bx.parse().unwrap();
        let by = by.parse().unwrap();
        let pricex = pricex.parse().unwrap();
        let pricey = pricey.parse().unwrap();
        games.push([(ax, ay), (bx, by), (pricex, pricey)]);
    }
    games
}

fn gcd(mut a: isize, mut b: isize) -> isize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn solve() {
    let games = parse();
    let mut cost = 0;
    for [(ax, ay), (bx, by), (px, py)] in games {
        // px = n * ax + m * bx
        // py = n * ay * m * by
        let (px, py) = (px + 10000000000000, py + 10000000000000);
        let q = ay / gcd(ax, ay);
        let r = ax / gcd(ax, ay);
        let lhs = q * px - r * py;
        let rhs = q * bx - r * by; // * m
        if rhs == 0 {
            panic!("Assumption that vectors are not colinear failed!");
        }
        if lhs % rhs == 0 {
            let m = lhs / rhs;
            let n = (px - m * bx) / ax;
            cost += n * 3 + m;
        }
    }
    println!("{cost}");
}
