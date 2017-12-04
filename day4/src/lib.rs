use std::collections::HashSet;
use std::iter::FromIterator;

pub fn parse_input(input: &str) -> Vec<Vec<&str>> {
    input.lines()
        .map(|line| line.split_whitespace().collect())
        .collect()
}

fn process_line(line: &Vec<&str>) -> bool {
    HashSet::<&str>::from_iter(line.iter().cloned()).len() == line.len()
}

pub fn process(data: Vec<Vec<&str>>) -> u32 {
    data.iter()
        .map(process_line)
        .filter(|&line| line)
        .count() as u32
}

#[cfg(test)]
mod tests;
