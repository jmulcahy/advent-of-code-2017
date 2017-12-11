pub fn process(input: &str) -> Option<(u32, u32)> {
    let mut score = 0;
    let mut adder = 1;
    let mut skip_ex = false;
    let mut skip_carrot = false;
    let mut skipped = 0;
    for c in input.trim().chars() {
        if skip_ex {
            skip_ex = false;
        } else if c == '!' {
            skip_ex = true;
        } else if c == '>' {
            skip_carrot = false;
        } else if skip_carrot {
            skipped += 1;
        } else {
            match c {
                '<' => skip_carrot = true,
                '{' => adder += 1,
                '}' => {adder -= 1; score += adder;},
                ',' => (),
                _ => return None,
            }
        }
    }
    Some((score, skipped))
}


#[cfg(test)]
mod tests;
