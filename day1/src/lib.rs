pub fn parse_input(input: &str) -> Option<Vec<u32>> {
    input.trim().chars()
        .map(|c| c.to_digit(10))
        .collect::<Option<Vec<u32>>>()
}

pub fn process1(data: &[u32]) -> u32 {
    let mut acc = 0;

    for i in 0..data.len() - 1 {
        let x1 = data[i];
        let x2 = data[i + 1];
        if x1 == x2 {
            acc += x1;
        }
    }
    if data[0] == data[data.len() - 1] {
        acc += data[0];
    }
    acc
}


#[cfg(test)]
mod tests;
