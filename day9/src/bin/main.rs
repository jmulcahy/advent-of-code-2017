extern crate day2;
extern crate day9;

fn main() {
    let filename = "input.txt";
    let input = day2::read_file(filename)
        .expect(&format!("couldn't open {}", filename));
    let result = day9::process(&input).expect("failed to process data after parsing");
    println!("Result 1:\n{}\nResult 2:\n{}", result.0, result.1);
}
