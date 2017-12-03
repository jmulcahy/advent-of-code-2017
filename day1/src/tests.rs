use parse_input;

#[test]
fn test_parse_input() {
    let expected: Vec<u32> = vec![9, 1, 2, 1, 2, 1, 2, 9];
    assert_eq!(Some(expected), parse_input("91212129 "));
}

use process1;
#[test]
fn test_process1() {
    let data: Vec<u32> = vec![9, 1, 2, 1, 2, 1, 2, 9];
    assert_eq!(9, process1(&data));
}


use process2;
#[test]
fn test_process2() {
    let data: Vec<u32> = vec![1, 2, 1, 3, 1, 4, 1, 5];
    assert_eq!(4, process2(&data));
}
