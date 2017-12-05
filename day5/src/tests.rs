use parse_input;

#[test]
fn test_parse_input() {
    let input = "2\n5\n0\n1\n-2";
    let expected = vec![2, 5, 0, 1, -2];
    assert_eq!(Ok(expected), parse_input(input));
}

use process1;

#[test]
fn test_process1() {
    let data: Vec<i32> = vec![0, 3, 0, 1, -3];
    assert_eq!(5, process1(&data));
}

use process2;

#[test]
fn test_process2() {
    let data: Vec<i32> = vec![0, 3, 0, 1, -3];
    assert_eq!(10, process2(&data));
}
