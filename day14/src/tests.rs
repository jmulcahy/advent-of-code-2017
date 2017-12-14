use count_bits;

#[test]
fn test_count_bits() {
    let hex_string = "ff";
    assert_eq!(Ok(8), count_bits(hex_string));
}

use process1;

#[test]
fn test_process1() {
    let input = "flqrgnkx";
    assert_eq!(Ok(8108), process1(input));
}
