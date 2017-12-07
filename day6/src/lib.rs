use std::collections::HashMap;

pub fn parse_input(input: &str) -> Result<Vec<u32>, std::num::ParseIntError> {
    input.split_whitespace().map(|bank| bank.parse()).collect()
}

pub fn update_banks(banks: &[u32]) -> Option<Vec<u32>> {
    let (max_index, max) = banks.iter()
        .enumerate()
        .rev()
        .max_by(|&(_, x), &(_, y)| x.cmp(y))?;
    Some(banks.iter()
        .enumerate()
        .map(|(j, y)| {
            let mut bank = *y;
            let extras = *max as usize % banks.len();
            if j == max_index {
                bank = 0;
            };
            if (j + banks.len() - max_index - 1) % banks.len() < extras {
                bank += 1;
            };
            bank + (max / banks.len() as u32)
        })
         .collect())
}

pub fn process(data: &[u32]) -> Option<(usize, usize)> {
    let mut seen = HashMap::new();
    seen.insert(data.to_vec(), 0);
    let mut current = update_banks(data)?;

    while !seen.contains_key(&current) {
        let it = seen.len() + 1;
        let next = update_banks(&current)?;
        seen.insert(current, it);
        current = next;
    }

    Some((seen.len(), seen.len() + 1 - seen.get(&current)?))
}

#[cfg(test)]
mod tests;
