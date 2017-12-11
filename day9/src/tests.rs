use process;

#[test]
fn test_process_1() {
    let input = "{}";
    assert_eq!(Some((1, 0)), process(input));
}

#[test]
fn test_process_2() {
    let input = "{{{}}}";
    assert_eq!(Some((6, 0)), process(input));
}

#[test]
fn test_process_3() {
    let input = "{{},{}}";
    assert_eq!(Some((5, 0)), process(input));
}

#[test]
fn test_process_4() {
    let input = "{{{},{},{{}}}}";
    assert_eq!(Some((16, 0)), process(input));
}

#[test]
fn test_process_5() {
    let input = "{<a>,<a>,<a>,<a>}";
    assert_eq!(Some((1, 4)), process(input));
}

#[test]
fn test_process_9() {
    let input = "{{<ab>},{<ab>},{<ab>},{<ab>}}";
    assert_eq!(Some((9, 8)), process(input));
}

#[test]
fn test_process_10() {
    let input = "{{<!!>},{<!!>},{<!!>},{<!!>}}";
    assert_eq!(Some((9, 0)), process(input));
}

#[test]
fn test_process_11() {
    let input = "{{<a!>},{<a!>},{<a!>},{<ab>}}";
    assert_eq!(Some((3, 17)), process(input));
}
