pub fn parse_input(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    input.lines().map(|line| line.parse()).collect()
}

pub fn process1(data: &[i32]) -> i32 {
    let mut steps = 0;
    let mut data_copy = data.to_vec();
    let mut pos = 0;
    while pos >= 0 && pos < data.len() as i32 {
        let current_pos = pos;
        pos += data_copy[pos as usize];
        data_copy[current_pos as usize] += 1;
        steps += 1;
    }
    steps
}

pub fn process2(data: &[i32]) -> i32 {
    let mut steps = 0;
    let mut data_copy = data.to_vec();
    let mut pos = 0;
    while pos >= 0 && pos < data.len() as i32 {
        let current_pos = pos as usize;
        pos += data_copy[pos as usize];
        if data_copy[current_pos] >= 3 {
            data_copy[current_pos] -= 1;
        } else {
            data_copy[current_pos] += 1;
        }
        steps += 1;
    }
    steps
}

#[cfg(test)]
mod tests;
