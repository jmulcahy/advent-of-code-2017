use read_file;
#[test]
fn test_read_file() {
    let filename = "test.txt";
    let input = "1 2 3\n1 2\n\n";
    let result = read_file(filename);
    assert!(result.is_ok(), result.err());
    assert_eq!(input, read_file(filename).unwrap());
}

use parse_input;
#[test]
fn test_parse_input() {
    let input = "1 2 3\n1 2\n\n";
    let data = vec![vec![1, 2, 3], vec![1, 2], Vec::new()];
    assert_eq!(data, parse_input(input));
}

use process;
#[test]
fn test_process() {
    let data = vec![vec![1, 2, 3], vec![1, 2], Vec::new()];
    assert_eq!(3, process(data.as_slice()));
}
