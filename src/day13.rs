use itertools::Itertools;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    fn new(tile_id: i64) -> Tile {
        match tile_id {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("bad tile"),
        }
    }

    fn draw(self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Wall => '#',
            Tile::Block => '.',
            Tile::Paddle => '_',
            Tile::Ball => 'o',
        }
    }
}

pub struct Game {
    computer: super::intcode::Computer,
    display: HashMap<(i64, i64), Tile>,
    score: i64,
}

impl Game {
    pub fn new(input: &str) -> Game {
        let mut computer = super::intcode::Computer::load(input);
        let display: HashMap<(i64, i64), Tile> = HashMap::new();
        computer.run();
        Game {
            computer: computer,
            display: display,
            score: 0,
        }
    }
    pub fn play(&mut self) {
loop {
            self.computer.run_with_input(0);
            self.read_output();
            self.draw();
        }

    }

    fn read_output(&mut self) {
        for (x, y, tile_id) in self.computer.outputs.iter().tuples() {
            if (*x, *y) == (-1, 0) {
                self.score = *tile_id
            } else {
                self.display.insert((*x, *y), Tile::new(*tile_id));
            }
        }
        self.computer.outputs = vec![];
    }

    pub fn draw(&self) {
        println!("Score: {}", self.score);
	println!("Blocks: {}", self.display.iter().filter(|(_,tile)| **tile == Tile::Block).count());

        for y in 0..20 {
            for x in 0..44 {
                print!("{}", self.display[&(x, y)].draw())
            }
            println!();
        }
    }
}
