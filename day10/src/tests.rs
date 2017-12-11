use process_once;

#[test]
fn test_process_once() {
    let data = [0, 1, 2, 3, 4];
    let expected = vec![2, 1, 0, 3, 4];
    assert_eq!(expected, process_once(&data, 0, 3));
}

use process1;

#[test]
fn test_process1() {
    let lengths = [3, 4, 1, 5];
    assert_eq!(12, process1(&lengths, 5));
}

use parse_input2;

#[test]
fn test_parse_input2() {
    let input = "1,2,3";
    let expected = vec![49, 44, 50, 44, 51];
    assert_eq!(expected, parse_input2(&input));
}

use dense_hash;

#[test]
fn test_dense_hash() {
    let sparse_hash = [65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22];
    assert_eq!(64, dense_hash(&sparse_hash));
}

use hex;

#[test]
fn test_hex() {
    let hash = [64, 7, 255];
    assert_eq!("4007ff", hex(&hash));
}

use process2;

#[test]
fn test_process2_1() {
    let input = "";
    let lengths = parse_input2(input);
    let expected = "a2582a3a0e66e6e86e3812dcb672a272";
    assert_eq!(expected, process2(&lengths));
}

#[test]
fn test_process2_2() {
    let input = "AoC 2017";
    let lengths = parse_input2(input);
    let expected = "33efeb34ea91902bb2f59c9920caa6cd";
    assert_eq!(expected, process2(&lengths));
}

#[test]
fn test_process2_3() {
    let input = "1,2,3";
    let lengths = parse_input2(input);
    let expected = "3efbe78a8d82f29979031a4aa0b16a9d";
    assert_eq!(expected, process2(&lengths));
}

#[test]
fn test_process2_4() {
    let input = "1,2,4";
    let lengths = parse_input2(input);
    let expected = "63960835bcdc130f0b66d7ff4f6a5a8e";
    assert_eq!(expected, process2(&lengths));
}
