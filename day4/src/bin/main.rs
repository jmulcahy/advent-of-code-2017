extern crate day2;
extern crate day4;

use std::error::Error;

fn main () {
    let filename = "input.txt";
    let input = match day2::read_file(filename) {
        Err(why) => panic!("couldn't open {}: {}", filename, why.description()),
        Ok(input) => input
    };

    println!("Result:\n{}", day4::process(day4::parse_input(&input)));
}
