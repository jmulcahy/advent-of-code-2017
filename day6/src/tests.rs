use parse_input;

#[test]
fn test_parse_input() {
    let input = "0\t2\t7\t0";
    let expected = vec![0, 2, 7, 0];
    assert_eq!(Ok(expected), parse_input(input));
}

use update_banks;

#[test]
fn test_update_banks() {
    let banks = vec![0, 2, 7, 0];
    let expected = vec![2, 4, 1, 2];
    assert_eq!(Some(expected), update_banks(&banks));
}

#[test]
fn test_update_banks_eq() {
    let banks = vec![0, 4, 4, 0];
    let expected = vec![1, 1, 5, 1];
    assert_eq!(Some(expected), update_banks(&banks));
}

use process;

#[test]
fn test_process1() {
    let data = vec![0, 2, 7, 0];
    assert_eq!(Some((5, 4)), process(&data));
}
