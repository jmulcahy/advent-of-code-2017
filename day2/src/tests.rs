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

use process1;
#[test]
fn test_process() {
    let data = vec![vec![1, 2, 3], vec![1, 2], Vec::new()];
    assert_eq!(3, process1(data.as_slice()));
}

use process_line2;
#[test]
fn test_process_line2() {
    let data = vec![5, 9, 2, 8];
    assert_eq!(Ok(4), process_line2(data.as_slice()));
}

#[test]
fn test_error_process_line2() {
    let data = vec![1, 1];
    assert_eq!(true, process_line2(data.as_slice()).is_err());
}

use process2;
#[test]
fn test_process2() {
    let data = vec![vec![5, 9, 2, 8],
                    vec![9, 4, 7, 3],
                    vec![3, 8, 6, 5]];
    assert_eq!(Ok(9), process2(data.as_slice()));
}
