use std::fs;
use std::io::{prelude::*, BufReader};
pub fn day1() -> i32 {
    let file = fs::File::open("data/day1.txt").unwrap();
    let reader = BufReader::new(file);

    let modules = reader.lines().map(|line| line.unwrap().parse().unwrap());
    modules.map(|module: i32| module / 3 - 2).sum()
}

pub fn day1_2() -> i32 {
    let file = fs::File::open("data/day1.txt").unwrap();
    let reader = BufReader::new(file);
    let modules = reader.lines().map(|line| line.unwrap().parse().unwrap());
    modules.map(|module: i32| total_fuel(module)).sum()
}

fn total_fuel(mass: i32) -> i32 {
    let mut fuel = 0;
    let mut additional_fuel = mass / 3 - 2;
    while additional_fuel > 0 {
        fuel += additional_fuel;
        additional_fuel = additional_fuel / 3 - 2;
    }
    fuel
}
