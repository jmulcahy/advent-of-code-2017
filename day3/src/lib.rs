#[derive(Debug, PartialEq, Eq)]
struct Cart {
    x: i32,
    y: i32,
}

fn go_right(pos: &Cart) -> Cart {
    Cart{x: pos.x + 1, y: pos.y}
}

fn go_up(pos: &Cart) -> Cart {
    Cart{x: pos.x, y: pos.y + 1}
}

fn go_left(pos: &Cart) -> Cart {
    Cart{x: pos.x - 1, y: pos.y}
}

fn go_down(pos: &Cart) -> Cart {
    Cart{x: pos.x, y: pos.y - 1}
}

fn next_pos(pos: &Cart) -> Result<Cart, Cart> {
    let &Cart{x, y} = pos;

    if y <= x && y <= -x {
        return Ok(go_right(pos))
    } else if y < x && y >= -x  {
        return Ok(go_up(pos))
    } else if y >= x && y > -x {
        return Ok(go_left(pos))
    } else if y > x && y <= -x {
        return Ok(go_down(pos))
    }

    Err(Cart{x: x, y: y})
}

fn l1_norm(pos: &Cart) -> i32 {
    println!("Coord: {:?}", pos);
    pos.x.abs() + pos.y.abs()
}

pub fn spiral_distance(n: i32) -> i32 {
    let mut current = Cart{x: 0, y: 0};
    for _i in 1..n {
        current = match next_pos(&current) {
            Ok(next) => next,
            Err(current) => panic!("Current pos: {:?}", current)
        };
    };
    println!("{:?}", current);
    l1_norm(&current)
}

#[cfg(test)]
mod tests;
