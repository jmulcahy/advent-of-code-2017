use Cart;
use next_pos;
use spiral_distance;

#[test]
fn test_next_pos2() {
    let current = Cart{x: 0, y: 0};
    let expected = Cart{x: 1, y: 0};
    assert_eq!(Ok(expected), next_pos(&current));
}

#[test]
fn test_next_pos3() {
    let current = Cart{x: 1, y: 0};
    let expected = Cart{x: 1, y: 1};
    assert_eq!(Ok(expected), next_pos(&current));
}

#[test]
fn test_next_pos4() {
    let current = Cart{x: 1, y: 1};
    let expected = Cart{x: 0, y: 1};
    assert_eq!(Ok(expected), next_pos(&current));
}

#[test]
fn test_next_pos5() {
    let current = Cart{x: 0, y: 1};
    let expected = Cart{x: -1, y: 1};
    assert_eq!(Ok(expected), next_pos(&current));
}

#[test]
fn test_next_pos6() {
    let current = Cart{x: -1, y: 1};
    let expected = Cart{x: -1, y: 0};
    assert_eq!(Ok(expected), next_pos(&current));
}

#[test]
fn test_next_pos7() {
    let current = Cart{x: -1, y: 0};
    let expected = Cart{x: -1, y: -1};
    assert_eq!(Ok(expected), next_pos(&current));
}

#[test]
fn test_next_pos8() {
    let current = Cart{x: -1, y: -1};
    let expected = Cart{x: 0, y: -1};
    assert_eq!(Ok(expected), next_pos(&current));
}

#[test]
fn test_next_pos9() {
    let current = Cart{x: 0, y: -1};
    let expected = Cart{x: 1, y: -1};
    assert_eq!(Ok(expected), next_pos(&current));
}

#[test]
fn test_next_pos10() {
    let current = Cart{x: 1, y: -1};
    let expected = Cart{x: 2, y: -1};
    assert_eq!(Ok(expected), next_pos(&current));
}

#[test]
fn test_spiral_distance3() {
    assert_eq!(2, spiral_distance(3));
}

#[test]
fn test_spiral_distance4() {
    assert_eq!(1, spiral_distance(4));
}

#[test]
fn test_spiral_distance6() {
    assert_eq!(1, spiral_distance(6));
}

#[test]
fn test_spiral_distance8() {
    assert_eq!(1, spiral_distance(8));
}

#[test]
fn test_spiral_distance11() {
    assert_eq!(2, spiral_distance(11));
}

#[test]
fn test_spiral_distance12() {
    assert_eq!(3, spiral_distance(12));
}

#[test]
fn test_spiral_distance23() {
    assert_eq!(2, spiral_distance(23));
}

#[test]
fn test_spiral_distance1024() {
    assert_eq!(31, spiral_distance(1024));
}

#[test]
fn test_spiral_distance() {
    assert_eq!(475, spiral_distance(277678));
}

use entry_less_than;

#[test]
fn test_part2_2() {
    assert_eq!(2, entry_less_than(1));
}

#[test]
fn test_part2_4() {
    assert_eq!(5, entry_less_than(4));
}

#[test]
fn test_part2_50() {
    assert_eq!(54, entry_less_than(50));
}
