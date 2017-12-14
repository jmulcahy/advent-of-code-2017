extern crate day10;

use std::num::ParseIntError;

fn count_bits(hex_string: &str) -> Result<usize, ParseIntError> {
    let count = hex_string.chars()
        .map(|c| {
            let num = u16::from_str_radix(&c.to_string(), 16)?;
            let binary_string = String::from(format!("{:b}", num));
            Ok(binary_string.matches('1').count())
        })
        .collect::<Result<Vec<usize>, ParseIntError>>()?.iter().sum();
    Ok(count)
}

pub fn process1(input: &str) -> Result<usize, ParseIntError> {
    let mut total = 0;
    for i in 0..128 {
        let hash = day10::process2(format!("{}-{}", input, i).as_bytes());
        total += count_bits(&hash)?;
    }
    Ok(total)
}

#[cfg(test)]
mod tests;
