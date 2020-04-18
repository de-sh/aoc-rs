// Day 1: Tyranny of the Rocket Equation (https://adventofcode.com/2019/day/1)
use std::io;
use std::io::BufRead;

fn fuel_per_mod(weight: u32) -> u32 {
    weight/3-2
}

pub fn sum_of_fuel_required() -> u32 {
    let stdin = io::stdin();
    let mut sum: u32 = 0;
    println!(r#"Input weights of modules as positive integers. 
To finish, press `ENTER` on empty line of input."#);
    for line in stdin.lock().lines() {
        let input = line.unwrap();
        let strip = input.trim();
        match strip.parse::<u32>() {
            Ok(i) => sum += fuel_per_mod(i),
            Err(..) => break,
        }
    }

    sum
}
