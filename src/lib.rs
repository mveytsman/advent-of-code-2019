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

pub fn day_2_1() {
    let mut computer = intcode::Computer::load("1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,9,19,1,5,19,23,1,6,23,27,1,27,10,31,1,31,5,35,2,10,35,39,1,9,39,43,1,43,5,47,1,47,6,51,2,51,6,55,1,13,55,59,2,6,59,63,1,63,5,67,2,10,67,71,1,9,71,75,1,75,13,79,1,10,79,83,2,83,13,87,1,87,6,91,1,5,91,95,2,95,9,99,1,5,99,103,1,103,6,107,2,107,13,111,1,111,10,115,2,10,115,119,1,9,119,123,1,123,9,127,1,13,127,131,2,10,131,135,1,135,5,139,1,2,139,143,1,143,5,0,99,2,0,14,0");
    computer.run();
    println!("{}", computer.memory.read(0));
}
pub fn day_2_2() -> intcode::Word {
    // lol lets search
    let input = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,9,19,1,5,19,23,1,6,23,27,1,27,10,31,1,31,5,35,2,10,35,39,1,9,39,43,1,43,5,47,1,47,6,51,2,51,6,55,1,13,55,59,2,6,59,63,1,63,5,67,2,10,67,71,1,9,71,75,1,75,13,79,1,10,79,83,2,83,13,87,1,87,6,91,1,5,91,95,2,95,9,99,1,5,99,103,1,103,6,107,2,107,13,111,1,111,10,115,2,10,115,119,1,9,119,123,1,123,9,127,1,13,127,131,2,10,131,135,1,135,5,139,1,2,139,143,1,143,5,0,99,2,0,14,0";
    for noun in 0..99 {
        for verb in 0..99 {
            let mut computer = intcode::Computer::load(input);
            computer.memory.write(1,  noun);
            computer.memory.write(2, verb);
            computer.run();
            if computer.memory.read(0) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    return -1;
}

pub fn day6_1() -> i32 {
    let input = fs::read_to_string("data/day6.txt").unwrap();
    day6::count_orbits(&input)
}

pub fn day6_2() -> i32 {
    let input = fs::read_to_string("data/day6.txt").unwrap();
    day6::count_transfers(&input, "YOU", "SAN")
}

pub fn day8_1() -> usize {
    let input = fs::read_to_string("data/day8.txt").unwrap();
    day8::day8_1(input.trim(), 25, 6)
}

pub fn day8_2() {
    let input = fs::read_to_string("data/day8.txt").unwrap();
    day8::collapse_layers(input.trim(), 25, 6);
}

pub mod day3;
pub mod day3_2;
pub mod day4;
pub mod day6;
pub mod day8;
pub mod intcode;
