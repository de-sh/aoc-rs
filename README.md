# Advent of Code 2019 in rust-lang

These are a collection of programs that solve questions from [Advent of Code 2019](https://adventofcode.com/2019/)

### Answers
- **[Day 1: The Tyranny of the Rocket Equation](https://adventofcode.com/2019/day/1)**: Find the weight of fuel that is required to launch a module. The logic to my solution is that since weight can't be negative and we seem to only consider integer values, we can utilise the `u32` type as argument and return variable of the `sum_of_fuel_required` function defined in the [`day_one`](src/day_one.rs) module.
