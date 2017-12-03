use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_file<P: AsRef<Path>>(filename: P) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|line| line
                      .split_whitespace()
                      .map(|num| num.parse().unwrap())
                      .collect()).collect()
}

pub fn process1(data: &[Vec<i32>]) -> i32 {
    data.iter()
        .map(|xs| xs.iter().max().unwrap_or(&0) - xs.iter().min().unwrap_or(&0))
        .sum()
}

fn process_line2(line: &[i32]) -> Result<i32, &'static str> {
    for x in line.iter() {
        for y in line.iter() {
            if x != y && x % y == 0 {
                return Ok(x / y);
            }
        }
    }
    Err("Row missing divisible values")
}

pub fn process2(data: &[Vec<i32>]) -> Result<i32, &'static str> {
    data.iter()
        .map(|xs| process_line2(xs.as_slice()))
        .collect::<Result<Vec<i32>, &'static str>>()
        .and_then(|xs| Ok(xs.iter().sum()))
}

#[cfg(test)]
mod tests;
