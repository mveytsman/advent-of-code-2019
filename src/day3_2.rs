use std::collections::HashSet;
use std::hash::{Hash, Hasher};
#[derive(Debug)]
enum MyError {
    ParseIntError(std::num::ParseIntError),
    UnparsableMove,
}

#[derive(Eq, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
    distance: u32,
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

#[derive(Debug, PartialEq)]
enum Move {
    Right(u32),
    Left(u32),
    Up(u32),
    Down(u32),
}

impl From<std::num::ParseIntError> for MyError {
    fn from(error: std::num::ParseIntError) -> Self {
        MyError::ParseIntError(error)
    }
}

fn parse_move(mv: &str) -> Result<Move, MyError> {
    let (direction, distance) = mv.split_at(1);
    let distance = distance.parse::<u32>()?;
    match direction {
        "R" => Ok(Move::Right(distance)),
        "L" => Ok(Move::Left(distance)),
        "U" => Ok(Move::Up(distance)),
        "D" => Ok(Move::Down(distance)),
        _ => Err(MyError::UnparsableMove),
    }
}

fn parse_moves(moves: &str) -> Result<HashSet<Point>, MyError> {
    let mut points = HashSet::new();
    let mut cur = Point {
        x: 0,
        y: 0,
        distance: 0,
    };
    for mv in moves.split(",") {
        let mv = parse_move(mv)?;
        match mv {
            Move::Right(distance) => {
                for _ in 0..distance as i32 {
                    cur = Point {
                        x: cur.x + 1,
                        y: cur.y,
                        distance: cur.distance + 1,
                    };
                    points.insert(cur);
                }
            }
            Move::Left(distance) => {
                for _ in 0..distance as i32 {
                    cur = Point {
                        x: cur.x - 1,
                        y: cur.y,
                        distance: cur.distance + 1,
                    };
                    points.insert(cur);
                }
            }
            Move::Up(distance) => {
                for _ in 0..distance as i32 {
                    cur = Point {
                        x: cur.x,
                        y: cur.y + 1,
                        distance: cur.distance + 1,
                    };
                    points.insert(cur);
                }
            }
            Move::Down(distance) => {
                for _ in 1..distance as i32 {
                    cur = Point {
                        x: cur.x,
                        y: cur.y - 1,
                        distance: cur.distance + 1,
                    };
                    points.insert(cur);
                }
            }
        }
    }
    Ok(points)
}

pub fn day3_2(input1: &str, input2: &str) -> u32 {
    //} -> Vec<Point> {
    let points1 = parse_moves(input1).unwrap();
    let points2 = parse_moves(input2).unwrap();
    let mut intersections = vec![];
    for point in points1 {
        match points2.get(&point) {
            Some(point2) => intersections.push(Point {
                x: point.x,
                y: point.y,
                distance: point.distance + point2.distance,
            }),
            None => (),
        }
    }
    intersections.sort_by(|a, b| a.distance.cmp(&b.distance));

    intersections[0].distance + 2
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_points() {
        assert_eq!(
            Point {
                x: 1,
                y: 2,
                distance: 3
            },
            Point {
                x: 1,
                y: 2,
                distance: 3
            }
        );
        assert_eq!(
            Point {
                x: 1,
                y: 2,
                distance: 3
            },
            Point {
                x: 1,
                y: 2,
                distance: 7
            }
        );
    }
    #[test]
    fn test_parse_move() {
        assert_eq!(Move::Right(12), parse_move("R12").unwrap());
        assert_eq!(Move::Left(144), parse_move("L144").unwrap());
        assert_eq!(Move::Up(1200), parse_move("U1200").unwrap());
        assert_eq!(Move::Down(1), parse_move("D1").unwrap());
    }

    #[test]
    fn test_parse_moves() {
        assert_eq!(
            610,
            day3_2(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83",
            )
        );
        assert_eq!(
            410,
            day3_2(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )
        )
        //day3_3("R3,D3", "D2,R4")
    }
}
