use std::collections::HashSet;
use std::ops;

pub type Coord = (usize, usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        *self = Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::SubAssign<Point> for Point {
    fn sub_assign(&mut self, rhs: Point) {
        *self = Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.cmp(&other.y)
            .then(self.y.cmp(&other.y))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Point {
    pub fn from(c: (isize, isize)) -> Self {
        Point {
            // x: c.0 as isize,
            // y: c.1 as isize,
            x: c.0,
            y: c.1,
        }
    }

    pub fn contained(&self, width: usize, height: usize) -> bool {
        if self.x < 0 || self.x >= width as isize || self.y < 0 || self.y >= height as isize {
            false
        } else {
            true
        }
    }

    pub fn manhattan(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn successors(&self, distance: usize) -> HashSet<Point> {
        let mut succs = HashSet::new();
        let mut queue = vec![*self];

        while !queue.is_empty() {
            let curr = queue.pop().unwrap();
            for direction in [
                (-1, 0),
                (1, 0),
                (0, -1),
                (0, 1),
            ]
                .iter() {
                let np = curr + Point { x: direction.0, y: direction.1 };
                if np.manhattan(self) <= distance && np != *self && !succs.contains(&np) {
                    succs.insert(np);
                    queue.push(np);
                }
            }
        }

        succs
    }
}

pub fn parse_with_lens<'a, V, F>(
    lines: &'a str,
    f: &'a F,
) -> (Coord, impl Iterator<Item = ((isize, isize), V)> + 'a)
where
    F: Fn(u8) -> V,
{
    let y_len = lines.lines().count();
    let x_len = lines.lines().next().map(|s| s.trim().len()).unwrap();
    let it = lines.lines().enumerate().flat_map(move |(y, line)| {
        line.trim()
            .bytes()
            .enumerate()
            .map(move |(x, b)| ((x as isize, y as isize), f(b)))
    });
    ((x_len, y_len), it)
}
