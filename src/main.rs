mod day_one;
use day_one::sum_of_fuel_required;

fn main() {
    let weight: u32 = 1969;
    println!("weight of module: {} \nfuel required: {}", weight, sum_of_fuel_required(weight));
}
