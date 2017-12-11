use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Func {
    Inc,
    Dec,
}

#[derive(Debug, PartialEq, Eq)]
enum Cond {
    Lt,
    Lte,
    Eq,
    Gte,
    Gt,
    Neq,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    target_reg: String,
    func: Func,
    func_arg: i32,
    cond_reg: String,
    cond: Cond,
    cond_arg: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseInstError {
    Incomplete,
    ParseIntError(ParseIntError),
    BadFunction,
    BadConditional,
}

impl From<ParseIntError> for ParseInstError {
    fn from(err: ParseIntError) -> ParseInstError {
        ParseInstError::ParseIntError(err)
    }
}


impl FromStr for Instruction {
    type Err = ParseInstError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut sp = line.trim().split_whitespace();
        let target_reg = sp.next().ok_or(ParseInstError::Incomplete)?.to_string();
        let func_str = sp.next().ok_or(ParseInstError::Incomplete)?;
        let func = match func_str {
            "inc" => Func::Inc,
            "dec" => Func::Dec,
            _ => return Err(ParseInstError::BadFunction),
        };
        let func_arg = sp.next()
            .ok_or(ParseInstError::Incomplete)?.parse()?;
        sp.next().ok_or(ParseInstError::Incomplete)?; // skip the if
        let cond_reg = sp
            .next().ok_or(ParseInstError::Incomplete)?.to_string();
        let cond_str = sp.next().ok_or(ParseInstError::Incomplete)?;
        let cond = match cond_str {
            "<" => Cond::Lt,
            "<=" => Cond::Lte,
            "==" => Cond::Eq,
            ">=" => Cond::Gte,
            ">" => Cond::Gt,
            "!=" => Cond::Neq,
            _ => return Err(ParseInstError::BadConditional),
        };
        let cond_arg = sp.next().ok_or(ParseInstError::Incomplete)?.parse()?;
        Ok(Instruction{target_reg, func, func_arg,
                       cond_reg, cond, cond_arg})
    }
}

pub fn parse_input(input: &str) -> Result<Vec<Instruction>, ParseInstError> {
    input.lines().map(|line| line.parse()).collect()
}

pub fn process(data: &[Instruction]) -> Option<(i32, i32)> {
    let mut regs = HashMap::new();
    let mut max_ever = None;
    for inst in data.iter() {
        let cond_reg = *regs.entry(&inst.cond_reg).or_insert(0);
        let cond = match inst.cond {
            Cond::Lt => cond_reg < inst.cond_arg,
            Cond::Lte => cond_reg <= inst.cond_arg,
            Cond::Eq => cond_reg == inst.cond_arg,
            Cond::Gte => cond_reg >= inst.cond_arg,
            Cond::Gt => cond_reg > inst.cond_arg,
            Cond::Neq => cond_reg != inst.cond_arg,
        };
        if cond {
            let target_reg = &inst.target_reg;
            let reg_val = *regs.entry(target_reg).or_insert(0);
            match inst.func {
                Func::Inc => regs.insert(target_reg, reg_val + inst.func_arg),
                Func::Dec => regs.insert(target_reg, reg_val - inst.func_arg),
            };
        }
        let current_max = regs.values().max().cloned();
        if current_max > max_ever {
            max_ever = current_max
        }
    }
    let results = (regs.values().max().cloned(), max_ever);
    match results {
        (None, _) => None,
        (_, None) => None,
        (max, max_ever) => Some((max.unwrap(), max_ever.unwrap())),
    }
}

#[cfg(test)]
mod tests;
