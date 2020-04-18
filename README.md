# Advent of Code 2019 in rust-lang

These are a collection of programs that solve questions from [Advent of Code 2019](https://adventofcode.com/2019/)

### Answers
- **[Day 1: The Tyranny of the Rocket Equation](https://adventofcode.com/2019/day/1)**: Find the weight of fuel that is required to launch a rocket. Weight of each consequent module is input as a line and the cumulative answer is to be output as a single value to be print in `main()`. The logic to my solution is that since weight can't be negative and we seem to only consider integer values, we can utilise the `u32` type as argument and return variable of the `fuel_per_mod` function that is called by `sum_of_fuel_required` function on encountering a module weight as input by the user. Both functions are defined in the [`day_one`](src/day_one.rs) module.
