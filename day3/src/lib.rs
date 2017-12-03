use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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
    l1_norm(&current)
}

fn insert_on_board(board: &HashMap<Cart, i32>, pos: &Cart) -> i32 {
    let mut acc = 0;
    for i in -1..2 {
        for j in -1..2 {
            acc += match board.get(&Cart{x: pos.x + i, y: pos.y + j}) {
                Some(&val) => val,
                None => 0
            };
        };
    };
    acc
}

pub fn entry_less_than(n: i32) -> i32 {
    let mut board = HashMap::new();
    let mut current = Cart{x: 0, y: 0};
    board.insert(current, 1i32);

    let mut i = 0;
    while i <= n {
        current = match next_pos(&current) {
            Ok(next) => next,
            Err(current) => panic!("Current pos: {:?}", current)
        };
        i = insert_on_board(&mut board, &current);
        board.insert(current, i);
    }
    i
}

#[cfg(test)]
mod tests;
