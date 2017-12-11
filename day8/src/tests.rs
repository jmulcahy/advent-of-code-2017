use Instruction;
use Func;
use Cond;

#[test]
fn test_from_str_single() {
    let input = "b inc 5 if a > 1";
    let expected = Instruction{target_reg: String::from("b"), func: Func::Inc,
                               func_arg: 5, cond_reg: String::from("a"), cond: Cond::Gt,
                               cond_arg: 1};
    assert_eq!(Ok(expected), input.parse());
}

use parse_input;

#[test]
fn test_from_str_multiple() {
    let input = " b inc 5 if a > 1 \n a inc 1 if b < 5 ";
    let one = Instruction{target_reg: String::from("b"), func: Func::Inc,
                          func_arg: 5, cond_reg: String::from("a"), cond: Cond::Gt,
                          cond_arg: 1};
    let two = Instruction{target_reg: String::from("a"), func: Func::Inc,
                          func_arg: 1, cond_reg: String::from("b"), cond: Cond::Lt,
                          cond_arg: 5};
    assert_eq!(Ok(vec![one, two]), parse_input(input));
}

use process;
#[test]
fn test_process() {
    let input = "b inc 5 if a > 1
        a inc 1 if b < 5
        c dec -10 if a >= 1
        c inc -20 if c == 10";
    let data = parse_input(input).unwrap();
    assert_eq!(Some((1, 10)), process(&data));
}
