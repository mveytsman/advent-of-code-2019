use std::collections::HashMap;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}

struct Robot {
    location: (i64, i64),
    direction: Direction,
}

impl Robot {
    fn new() -> Robot {
        Robot {
            location: (0, 0),
            direction: Direction::Up,
        }
    }
    fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }

    fn turn_left(&mut self) {
        self.direction = self.direction.turn_left();
    }

    fn advance(&mut self) {
        let (x, y) = self.location;
        self.location = match self.direction {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y +1),
            Direction::Left => (x - 1, y),
        }
    }
}

pub fn paint_panels(input: &str, starting_color: i64) -> HashMap<(i64,i64),i64>{
    let mut computer = super::intcode::Computer::load(input);
    let mut panels : HashMap<(i64,i64),i64> = HashMap::new();
    panels.insert((0,0),starting_color);
    let mut robot = Robot::new();
    while !computer.halted() {
	let cur_color = panels.entry(robot.location).or_insert(0);
	computer.run_with_input(*cur_color);

	let direction = computer.outputs.pop().unwrap();
	let color = computer.outputs.pop().unwrap();

	panels.insert(robot.location, color);
	if direction == 0 {
	    robot.turn_left()
	} else {
	    robot.turn_right()
	}
	robot.advance();

    }
    panels
}


pub fn print_panels(panels: HashMap<(i64,i64),i64>) {
    let min_x = *panels.keys().map (|(x,y)| x).min().unwrap();
    let max_x = *panels.keys().map (|(x,y)| x).max().unwrap();
    let min_y = *panels.keys().map (|(x,y)| y).min().unwrap();
    let max_y = *panels.keys().map (|(x,y)| y).max().unwrap();
	for y in min_y..max_y+1 {
    for x in min_x..max_x+1 {
	    match panels.get(&(x,y)) {
		Some(1) => print!("x"),
		Some(0) => print!(" "),
		Some(_) => panic!("OH"),
		None => print!(" ")
	    }
	}
	print!("\n");
    }
}
