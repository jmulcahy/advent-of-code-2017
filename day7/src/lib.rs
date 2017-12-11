use std::str::FromStr;
use std::num::ParseIntError;
extern crate regex;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Program {
    name: String,
    weight: u32,
    above: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseProgError {
    Incomplete,
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for ParseProgError {
    fn from(err: ParseIntError) -> ParseProgError {
        ParseProgError::ParseIntError(err)
    }
}

impl FromStr for Program {
    type Err = ParseProgError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"\s\(|\)\s->\s|\)|,\s").unwrap();
        let mut sp = re.split(line.trim());
        let name = sp.next().ok_or(ParseProgError::Incomplete)?.to_string();
        let weight = sp.next().ok_or(ParseProgError::Incomplete)?.parse()?;
        let above = sp.map(|s| s.to_string()).collect();
        Ok(Program{name, weight, above})
    }
}

pub fn parse_string(input: &str) -> Result<Vec<Program>, ParseProgError> {
    input.lines()
        .map(|line|
            line.parse())
    .collect::<Result<Vec<Program>, ParseProgError>>()
}

pub fn find_root(programs: &[Program]) -> Option<String> {
    let mut children = HashSet::new();
    programs.iter().for_each(|p| p.above.iter()
                          .for_each(|c| {children.insert(c);}));
    for program in programs.iter() {
        if !children.contains(&program.name) {
            return Some(program.name.clone());
        }
    }
    None
}

#[derive(Debug)]
pub struct TreeNode<T> {
    value: T,
    children: Vec<TreeNode<T>>,
}

fn build(name: &str, program_map: &HashMap<String, &Program>)
         -> Option<TreeNode<Program>> {
    let program = program_map.get(name)?;
    if program.above.is_empty() {
        return Some(TreeNode{value: program.clone().clone(), children: Vec::new()});
    }
    let mut aboves = Vec::new();
    for above in program.above.iter() {
        aboves.push(build(above, program_map)?);
    }
    return Some(TreeNode{value: program.clone().clone(), children: aboves});
}

fn build_tree(programs: &[Program]) -> Option<TreeNode<Program>> {
    let mut program_map = HashMap::new();
    programs.iter().for_each(|p| {program_map.insert(p.name.clone(), p);});
    let root_name = find_root(&programs)?;
    build(&root_name, &program_map)
}

fn check_tree(node: &TreeNode<Program>) -> u32 {
    if node.children.is_empty() {
        return node.value.weight;
    }
    let weights = node.children.iter().map(|c| check_tree(c)).collect::<Vec<u32>>();

    let mut all_eq = true;
    let mut last = weights[0];
    for &weight in weights[1..].iter() {
        if weight != last {
            all_eq = false;
        }
        last = weight;
    }
    if !all_eq {
        let names = node.children.iter().map(|c| c.value.name.clone()).collect::<Vec<String>>();
        println!("{:?}, {:?}", weights, names);
    }

    weights.iter().sum::<u32>() + node.value.weight
}

pub fn check_balance(programs: &[Program]) -> Option<u32> {
    let tree = build_tree(programs)?;
    Some(check_tree(&tree))
}

#[cfg(test)]
mod tests;
