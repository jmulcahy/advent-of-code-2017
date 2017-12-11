extern crate day2;
extern crate day7;

fn main() {
    let filename = "input.txt";
    let input = day2::read_file(filename)
        .expect(&format!("couldn't open {}", filename));
    let programs = day7::parse_string(&input).expect("parsing failed");
    let result = day7::find_root(&programs).expect("there is no root");
    println!("Result 1:\n{}", result);
    day7::check_balance(&programs);
}
