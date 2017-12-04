use parse_input;

#[test]
fn test_parse_input() {
    let input = "aa bb cc dd ee\naa bb cc dd aa\naa bb cc dd aaa";
    let expected = vec![vec!["aa", "bb", "cc", "dd", "ee"],
                        vec!["aa", "bb", "cc", "dd", "aa"],
                        vec!["aa", "bb", "cc", "dd", "aaa"]];
    assert_eq!(expected, parse_input(input));
}

use process;

#[test]
fn test_process() {
    let data = vec![vec!["aa", "bb", "cc", "dd", "ee"],
                    vec!["aa", "bb", "cc", "dd", "aa"],
                    vec!["aa", "bb", "cc", "dd", "aaa"]];
    assert_eq!(2, process(data));
}
