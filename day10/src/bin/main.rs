extern crate day2;
extern crate day10;

fn main() {
    let filename = "input.txt";
    let input = day2::read_file(filename)
        .expect(&format!("couldn't open {}", filename));
    let lengths1 = day10::parse_input1(&input).expect("failed to parse input");
    let result1 = day10::process1(lengths1.as_slice(), 256);
    println!("Result 1:\n{}", result1);

    let lengths2 = day10::parse_input2(&input);
    let result2 = day10::process2(&lengths2);
    println!("Result 2:\n{}", result2);
}
