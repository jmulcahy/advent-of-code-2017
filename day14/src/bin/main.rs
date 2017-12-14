extern crate day14;

fn main() {
    let input = "oundnydw";
    let result = day14::process1(input).expect("failed to process1");
    println!("Result 1:\n{}", result);
}
