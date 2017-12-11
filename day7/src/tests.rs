use Program;
use parse_string;

#[test]
fn test_parse_string1() {
    let input =
        "fwft (72) -> ktlj";
    let expected =
        vec![Program{name: String::from("fwft"),
                     weight: 72, above: vec![String::from("ktlj")]}];
    assert_eq!(Ok(expected), parse_string(input));
}

#[test]
fn test_parse_string2() {
    let input =
        "fwft (72) -> ktlj, cntj, xhth\nqoyq (66)\npadx (45) -> pbga, havc, qoyq";
    let expected =
        vec![Program{name: String::from("fwft"), weight: 72,
                     above: vec![String::from("ktlj"), String::from("cntj"),
                                 String::from("xhth")]},
             Program{name: String::from("qoyq"), weight: 66, above: vec![]},
             Program{name: String::from("padx"), weight: 45,
                     above: vec![String::from("pbga"), String::from("havc"),
                                 String::from("qoyq")]}];
    assert_eq!(Ok(expected), parse_string(input));
}

use find_root;

#[test]
fn test_find_root() {
    let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
    let programs = parse_string(input).unwrap();
    assert_eq!(Some(String::from("tknk")), find_root(&programs));
}

use check_balance;

#[test]
fn test_tree() {
    let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
    let programs = parse_string(input).unwrap();
    assert_eq!(Some(0), check_balance(&programs));
}
