// Day 1: Tyranny of the Rocket Equation (https://adventofcode.com/2019/day/1)
use std::io;
use std::io::BufRead;

pub fn sum_of_fuel_required() -> u32 {
    let stdin = io::stdin();
    let mut sum: u32 = 0;
    println!(r#"Input weights of modules as positive integers. 
To finish, press `ENTER` on empty line of input."#);
    for line in stdin.lock().lines() {
        let input = line.unwrap();
        let strip = input.trim();
        match strip.parse::<u32>() {
            Ok(i) => sum += i/3-2,
            Err(..) => break,
        }
    }
    
    let mut fuel = sum.clone();
    let mut offset = fuel/3-2;
    // Part Two: Fuel to lift fuel
    while offset > 0 {
        offset = fuel/3-2;
        fuel-= offset;
        sum+= offset;
    }
    
    sum
}
