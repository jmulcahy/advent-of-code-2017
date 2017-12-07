extern crate day2;
extern crate day6;

use std::error::Error;

fn main() {
    let filename = "input.txt";
    let input = match day2::read_file(filename) {
        Err(why) => panic!("couldn't open {}: {}", filename, why.description()),
        Ok(input) => input
    };
    let data = match day6::parse_input(&input) {
        Err(why) => panic!("failed to parse after reading, why: {}", why.description()),
        Ok(data) => data,
    };
    let (p1, p2) = match day6::process(&data) {
        Some((one, two)) => (one, two),
        _ => panic!("failed to process data after parsing"),
    };
    println!("Result 1:\n{}\nResult 2:\n{}", p1, p2);
}
