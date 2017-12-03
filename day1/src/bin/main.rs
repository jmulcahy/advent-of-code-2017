extern crate day2;
extern crate day1;

use std::error::Error;

fn main() {
    let filename = "input.txt";
    let input = match day2::read_file(filename) {
        Err(why) => panic!("couldn't open {}: {}", filename, why.description()),
        Ok(input) => input
    };
    let data = match day1::parse_input(&input) {
        Some(data) => data,
        _ => panic!("failed to parse input after reading"),
    };
    println!("Result 1:\n{}", day1::process1(data.as_slice()));
    println!("Result 2:\n{}", day1::process2(data.as_slice()));
}
