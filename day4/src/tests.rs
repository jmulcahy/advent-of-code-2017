use parse_input;

#[test]
fn test_parse_input() {
    let input = "aa bb cc dd ee\naa bb cc dd aa\naa bb cc dd aaa";
    let expected = vec![vec!["aa", "bb", "cc", "dd", "ee"],
                        vec!["aa", "bb", "cc", "dd", "aa"],
                        vec!["aa", "bb", "cc", "dd", "aaa"]];
    assert_eq!(expected, parse_input(input));
}

use process1;

#[test]
fn test_process1() {
    let data = vec![vec!["aa", "bb", "cc", "dd", "ee"],
                    vec!["aa", "bb", "cc", "dd", "aa"],
                    vec!["aa", "bb", "cc", "dd", "aaa"]];
    assert_eq!(2, process1(data));
}

use process2;
#[test]
fn test_process2() {
    let data = vec![vec!["abcde", "fghij"],
                    vec!["abcde", "xyz", "ecdab"],
                    vec!["a", "ab", "abc", "abd", "abf", "abj"],
                    vec!["iiii", "oiii", "ooii", "oooi", "oooo"],
                    vec!["oiii", "ioii", "iioi", "iiio"]];
    assert_eq!(3, process2(data));
}
