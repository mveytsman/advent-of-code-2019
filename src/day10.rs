extern crate num;
use num::integer::gcd;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;
use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Slope {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Asteroid {
    num_visible: i32,
}

impl Asteroid {
    fn new(i: i32) -> Self {
        Asteroid { num_visible: i }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Asteroids {
    data: HashMap<Point, Asteroid>,
}
impl Asteroids {
    fn new() -> Asteroids {
        Asteroids {
            data: HashMap::new(),
        }
    }
    pub fn from_input(input: &str) -> Asteroids {
        let mut coordinates = HashSet::new();

        for (i, line) in input.split("\n").enumerate() {
            for (j, chr) in line.chars().enumerate() {
                if chr == '#' {
                    let point = Point::new(i, j);
                    coordinates.insert(point);
                }
            }
        }

        let mut data = HashMap::new();

        for (i, pt1) in coordinates.iter().enumerate() {
            for pt2 in coordinates.iter().skip(i + 1) {
                if visible(&coordinates, pt1, pt2) {
                    data.entry(*pt1)
                        .and_modify(|ast: &mut Asteroid| ast.num_visible += 1)
                        .or_insert(Asteroid::new(1));
                    data.entry(*pt2)
                        .and_modify(|ast: &mut Asteroid| ast.num_visible += 1)
                        .or_insert(Asteroid::new(1));
                }
            }
        }

        Asteroids { data: data }
    }

    pub fn max_visible(&self) -> (Point, Asteroid) {
        let (x, y) = self
            .data
            .iter()
            .max_by(|(_, a), (_, b)| a.num_visible.cmp(&b.num_visible))
            .unwrap();
        (*x, *y)
    }
}

fn visible(coordinates: &HashSet<Point>, pt1: &Point, pt2: &Point) -> bool {
    let slope = pt1.slope_to(*pt2);
    let mut cur = *pt1 + slope;
    while cur != *pt2 {
        if coordinates.contains(&cur) {
            return false;
        }
        cur = cur + slope;
    }
    true
}

impl Add<Slope> for Point {
    type Output = Point;

    fn add(self, slope: Slope) -> Point {
        Point {
            x: self.x + slope.x,
            y: self.y + slope.y,
        }
    }
}

impl Point {
    fn new<T>(x: T, y: T) -> Point
    where
        T: TryInto<i32>,
        <T as std::convert::TryInto<i32>>::Error: std::fmt::Debug,
    {
        Point {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
        }
    }
    fn slope_to(self, other: Point) -> Slope {
        let x = other.x - self.x;
        let y = other.y - self.y;
        let g = gcd(x, y);

        Slope { x: x / g, y: y / g }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_point() {
        assert_eq!(Point::new(-13, 12), Point { x: -13, y: 12 });
        assert_eq!(
            Point { x: 12, y: 10 }.slope_to(Point { x: 6, y: 2 }),
            Slope { x: -3, y: -4 }
        );
        assert_eq!(
            Point { x: 12, y: 10 } + Slope { x: -3, y: -4 },
            Point { x: 9, y: 6 }
        );
    }

    #[test]
    fn test_input() {
        let input = ".#..#
.....
#####
....#
...##";
        let coordinates = vec![
            ((0, 1), 7),
            ((0, 4), 7),
            ((2, 0), 6),
            ((2, 1), 7),
            ((2, 2), 7),
            ((2, 3), 7),
            ((2, 4), 5),
            ((3, 4), 7),
            ((4, 3), 8),
            ((4, 4), 7),
        ];
        let asteroids: HashMap<Point, Asteroid> = coordinates
            .iter()
            .map(|((x, y), n)| (Point::new(*x, *y), Asteroid::new(*n)))
            .collect();
        assert_eq!(Asteroids { data: asteroids }, Asteroids::from_input(input));
        assert_eq!(
            (Point::new(4, 3), Asteroid::new(8)),
            Asteroids::from_input(input).max_visible()
        );

        let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

        assert_eq!(
            (Point::new(13, 11), Asteroid::new(210)),
            Asteroids::from_input(input).max_visible()
        );
    }
}
