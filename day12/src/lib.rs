extern crate regex;

use std::collections::HashMap;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Program {
    name: String,
    connects: Vec<String>,
}

#[derive(Debug)]
pub enum ParseProgError {
    Incomplete,
}

impl FromStr for Program {
    type Err = ParseProgError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"\s<->\s|,\s").unwrap();
        let mut sp = re.split(line.trim());
        let name = sp.next().ok_or(ParseProgError::Incomplete)?.to_string();
        let connects = sp.map(|s| s.to_string()).collect();
        Ok(Program{name, connects})
    }
}

pub fn parse_input(input: &str)
               -> Result<HashMap<String, Program>, ParseProgError> {
    input.lines()
        .map(|line| {
            let program = line.trim().parse::<Program>()?;
            Ok((program.name.clone(), program))
        })
        .collect()
}

fn find_group(data: &HashMap<String, Program>, group: &HashSet<String>,
                  choice: &str) -> Option<HashSet<String>> {
    let mut new_group = group.clone();
    for connect in data.get(choice)?.connects.iter() {
        if new_group.insert(connect.clone()) {
            let next = find_group(data, &new_group, connect)?;
            new_group = new_group.union(&next).cloned().collect();
        }
    }
    Some(new_group)
}

pub fn process1(data: &HashMap<String, Program>, choice: &str) -> Option<usize> {
    let node = data.get(choice)?;
    let mut group = HashSet::new();
    group.insert(node.name.clone());
    group = find_group(data, &group, &node.name)?;
    Some(group.len())
}

pub fn process2(data: &HashMap<String, Program>) -> Option<usize> {
    let mut grouped = HashSet::new();
    let mut num_groups = 0;
    for name in data.keys() {
        if !grouped.contains(name) {
            grouped = grouped.union(&find_group(data, &grouped, name)?)
                .cloned().collect();
            num_groups += 1;
        }
    }
    Some(num_groups)
}

#[cfg(test)]
mod tests;
