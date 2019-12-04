use std::cmp;
use std::cmp::Ordering;
#[derive(Debug, Copy, Clone)]
enum Segment {
    HorizontalSegment { x_1: i32, x_2: i32, y: i32 },
    VerticalSegment { y_1: i32, y_2: i32, x: i32 },
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Point(i32, i32);

impl Point {
    fn manhattan_distance(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.manhattan_distance().cmp(&other.manhattan_distance())
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Segment {
    fn new(a: Point, b: Point) -> Self {
        if a.0 == b.0 {
            Segment::VerticalSegment {
                y_1: cmp::min(a.1, b.1),
                y_2: cmp::max(a.1, b.1),
                x: a.0,
            }
        } else {
            Segment::HorizontalSegment {
                x_1: cmp::min(a.0, b.0),
                x_2: cmp::max(a.0, b.0),
                y: a.1,
            }
        }
    }
    fn intersect(self, other: Self) -> Option<Point> {
        match self {
            Self::HorizontalSegment { x_1, x_2, y } => match other {
                Self::HorizontalSegment { x_1, x_2, y } => None,
                Self::VerticalSegment { y_1, y_2, x } => {
                    if x_1 <= x && x <= x_2 && y_1 <= y && y <= y_2 {
                        Some(Point(x, y))
                    } else {
                        None
                    }
                }
            },
            Self::VerticalSegment { y_1, y_2, x } => match other {
                Self::HorizontalSegment { x_1, x_2, y } => {
                    if x_1 <= x && x <= x_2 && y_1 <= y && y <= y_2 {
                        Some(Point(x, y))
                    } else {
                        None
                    }
                }
                Self::VerticalSegment { y_1, y_2, x } => None,
            },
        }
    }
}

fn parse_input(input: &str) -> Vec<Segment> {
    let parts = input.split(",");
    let mut segments: Vec<Segment> = vec![];
    let mut start = Point(0, 0);
    for part in parts {
        let (direction, distance) = part.split_at(1);
        let distance = distance.parse::<i32>().unwrap();
        let end = match direction {
            "R" => Point(start.0 + distance, start.1),
            "L" => Point(start.0 - distance, start.1),
            "U" => Point(start.0, start.1 + distance),
            "D" => Point(start.0, start.1 - distance),
            _ => panic!("lol"),
        };

        segments.push(Segment::new(start, end));
        start = end;
    }
    segments
}

pub fn day3(input1: &str, input2: &str) -> i32 {
    let segments1 = parse_input(input1);
    let segments2 = parse_input(input2);
    let mut points = vec![];
    for s1 in segments1 {
        for s2 in segments2.clone() {
            match s1.intersect(s2) {
                Some(point) => points.push(point),
                None => (),
            }
        }
    }
    points.sort();
    if points[0] == Point(0, 0) {
        points[1].manhattan_distance()
    } else {
        points[0].manhattan_distance()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_point_sorting() {
        let mut points = vec![Point(1, 2), Point(-2, -2), Point(12, 15), Point(5, 5)];
        points.sort();
        assert_eq!(
            points,
            vec![Point(1, 2), Point(-2, -2), Point(5, 5), Point(12, 15)]
        )
    }

    #[test]
    fn test_intersect() {
        assert_eq!(
            None,
            Segment::HorizontalSegment {
                x_1: 0,
                x_2: 2,
                y: -3
            }
            .intersect(Segment::HorizontalSegment {
                x_1: 0,
                x_2: 2,
                y: -1
            })
        );
        assert_eq!(
            Some(Point(1, 2)),
            Segment::VerticalSegment {
                y_1: 0,
                y_2: 3,
                x: 1
            }
            .intersect(Segment::HorizontalSegment {
                x_1: 0,
                x_2: 2,
                y: 2
            })
        )
    }

    #[test]
    fn test_stuff() {
        let input1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let input2 = "U62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(159, day3(input1, input2));
        assert_eq!(
            135,
            day3(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )
        );
    }
}
