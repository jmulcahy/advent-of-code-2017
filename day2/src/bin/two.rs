extern crate day2;

use std::error::Error;

fn main() {
    let filename = "input.txt";
    let input = match day2::read_file(filename) {
        Err(why) => panic!("couldn't open {}: {}", filename, why.description()),
        Ok(input) => input
    };

    let result = match day2::process2(day2::parse_input(&input).as_slice()) {
        Err(why) => panic!(why),
        Ok(result) => result
    };

    println!("Result:\n{}", result);
}
