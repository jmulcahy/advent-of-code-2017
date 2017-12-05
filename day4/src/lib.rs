use std::collections::HashSet;
use std::iter::FromIterator;

pub fn parse_input(input: &str) -> Vec<Vec<&str>> {
    input.lines()
        .map(|line| line.split_whitespace().collect())
        .collect()
}

fn process_line1(line: &Vec<&str>) -> bool {
    HashSet::<&str>::from_iter(line.iter().cloned()).len() == line.len()
}

pub fn process1(data: Vec<Vec<&str>>) -> u32 {
    data.iter()
        .map(process_line1)
        .filter(|&line| line)
        .count() as u32
}

fn process_line2(line: &Vec<&str>) -> bool {
    HashSet::<Vec<u8>>::from_iter(line.iter()
                                .map(|word| {
                                    let mut copy = word.as_bytes().to_vec();
                                    copy.sort();
                                    copy
                                }))
        .len() == line.len()
}

pub fn process2(data: Vec<Vec<&str>>) -> u32 {
    data.iter()
        .map(process_line2)
        .filter(|&line| line)
        .count() as u32
}

#[cfg(test)]
mod tests;
