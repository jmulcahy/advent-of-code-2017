extern crate day2;
extern crate day5;

use std::error::Error;

fn main() {
    let filename = "input.txt";
    let input = match day2::read_file(filename) {
        Err(why) => panic!("couldn't open {}: {}", filename, why.description()),
        Ok(input) => input
    };
    let data = match day5::parse_input(&input) {
        Err(why) => panic!("failed to parse after reading, why: {}", why.description()),
        Ok(data) => data,
    };
    println!("Result 1:\n{}", day5::process1(&data));
    println!("Result 2:\n{}", day5::process2(&data));
}
