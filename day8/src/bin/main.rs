extern crate day2;
extern crate day8;

fn main() {
    let filename = "input.txt";
    let input = day2::read_file(filename)
        .expect(&format!("couldn't open {}", filename));
    let data = day8::parse_input(&input).expect("failed to parse data");
    let result = day8::process(&data).expect("failed to process data");
    println!("Result 1:\n{}\nResult 2:\n{}", result.0, result.1);
}
