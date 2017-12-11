use std::cmp;

fn move_hex(current: Option<((i32, i32), u32)>, direction: &str)
            -> Option<((i32, i32), u32)> {
    let (pos, max_dist) = current?;
    let new_pos = match direction {
        "n" => Some((pos.0, pos.1 + 2)),
        "ne" => Some((pos.0 + 2, pos.1 + 1)),
        "se" => Some((pos.0 + 2, pos.1 - 1)),
        "s" => Some((pos.0, pos.1 - 2)),
        "sw" => Some((pos.0 - 2, pos.1 - 1)),
        "nw" => Some((pos.0 - 2, pos.1 + 1)),
        _ => None
    }?;
    Some((new_pos, cmp::max(max_dist, shortest_dist(&new_pos))))
}

fn shortest_dist(pos: &(i32, i32)) -> u32 {
    let abs_pos = (pos.0.abs(), pos.1.abs());
    let nes = abs_pos.0 / 2;
    if abs_pos.1 - nes < 0 {
        return nes as u32;
    } else {
        return (nes + (abs_pos.1 - nes) / 2) as u32;
    }
}

pub fn get_shortest_dist(input: &str) -> Option<(u32, u32)> {
    let result = input.trim().split(',').fold(Some(((0, 0), 0)), move_hex)?;
    Some((shortest_dist(&result.0), result.1))
}

#[cfg(test)]
mod tests;
