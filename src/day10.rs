extern crate num;
use num::integer::gcd;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;
use std::f64;
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
    order_vaporized: i32,
}

impl Asteroid {
    fn new(i: i32) -> Self {
        Asteroid {
            num_visible: i,
            order_vaporized: -1,
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Asteroids {
    width: i32,
    height: i32,
    data: HashMap<Point, Asteroid>,
}
impl Asteroids {
    pub fn from_input(input: &str) -> Asteroids {
        let mut coordinates = HashSet::new();

        let lines: Vec<&str> = input.split("\n").collect();
        let height = lines.len();
        let width = lines[0].len();
        for (i, line) in lines.iter().enumerate() {
            for (j, chr) in line.chars().enumerate() {
                if chr == '#' {
                    let point = Point::new(j, i);
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

        Asteroids {
            data: data,
            height: height as i32,
            width: width as i32,
        }
    }

    pub fn max_visible(&self) -> (Point, Asteroid) {
        let (x, y) = self
            .data
            .iter()
            .max_by(|(_, a), (_, b)| a.num_visible.cmp(&b.num_visible))
            .unwrap();
        (*x, *y)
    }

    pub fn vaporize(&mut self) {
        let (start, _) = self.max_visible();
        let mut edge = Point::new(start.x, 0);
        let mut i = 1;

        let mut coordinates: Vec<&Point> = self.data.keys().filter(|x| *x != &start).collect();
        coordinates.sort_by(|a, b| {
            start
                .slope_to(**a)
                .angle()
                .partial_cmp(&start.slope_to(**b).angle())
                .unwrap()
        });
        for i in coordinates {
            dbg!(&start.slope_to(*i));
        }
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

fn first_visible(
    coordinates: &HashSet<&Point>,
    start: Point,
    slope: Slope,
    width: i32,
    height: i32,
) -> Option<Point> {
    let mut cur = start + slope;
    while cur.x >= 0 && cur.x < width && cur.y < height && cur.y > 0 {
        if coordinates.contains(&cur) {
            return Some(cur);
        }
        cur = cur + slope;
    }
    None
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

impl Slope {
    fn angle(&self) -> f64 {
        let ang = (self.y as f64).atan2(self.x as f64);
        // we want clockwise and starting from pi/4
        0.0 - (ang - f64::consts::FRAC_PI_2)
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

    fn next_edge(&self, width: i32, height: i32) -> Point {
        match self {
            Point { x: 0, y: 0 } => Point::new(1, 0),
            Point { x, y: 0 } if *x == width - 1 => Point::new(width - 1, 1),
            Point { x, y } if *x == width - 1 && *y == height - 1 => {
                Point::new(width - 2, height - 1)
            }
            Point { x: 0, y } if *y == height - 1 => Point::new(0, height - 2),
            Point { x, y: 0 } => Point::new(*x + 1, 0),
            Point { x, y } if *x == width - 1 => Point::new(width - 1, *y + 1),
            Point { x, y } if *y == height - 1 => Point::new(*x - 1, height - 1),
            Point { x, y } => Point::new(*x, *y - 1),
        }
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
            ((1, 0), 7),
            ((4, 0), 7),
            ((0, 2), 6),
            ((1, 2), 7),
            ((2, 2), 7),
            ((3, 2), 7),
            ((4, 2), 5),
            ((4, 3), 7),
            ((3, 4), 8),
            ((4, 4), 7),
        ];
        let asteroids: HashMap<Point, Asteroid> = coordinates
            .iter()
            .map(|((x, y), n)| (Point::new(*x, *y), Asteroid::new(*n)))
            .collect();
        assert_eq!(
            Asteroids {
                height: 5,
                width: 5,
                data: asteroids
            },
            Asteroids::from_input(input)
        );
        assert_eq!(
            (Point::new(3, 4), Asteroid::new(8)),
            Asteroids::from_input(input).max_visible()
        );

        let input = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
        assert_eq!(
            (Point::new(5, 8), Asteroid::new(33)),
            Asteroids::from_input(input).max_visible()
        );
	Asteroids::from_input(input).vaporize();

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
            (Point::new(11, 13), Asteroid::new(210)),
            Asteroids::from_input(input).max_visible()
        );


    }
}