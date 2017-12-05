extern crate day2;
extern crate day4;

use std::error::Error;

fn main () {
    let filename = "input.txt";
    let input = match day2::read_file(filename) {
        Err(why) => panic!("couldn't open {}: {}", filename, why.description()),
        Ok(input) => input
    };

    println!("Result 1:\n{}", day4::process1(day4::parse_input(&input)));
    println!("Result 2:\n{}", day4::process2(day4::parse_input(&input)));
}
