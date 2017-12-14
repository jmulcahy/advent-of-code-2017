use parse_input;
use process1;

#[test]
fn test_process1() {
    let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
    let data = parse_input(input).expect("failed to parse");
    assert_eq!(Some(6), process1(&data, "0"));
}
