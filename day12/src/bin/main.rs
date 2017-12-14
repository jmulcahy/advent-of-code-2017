extern crate day2;
extern crate day12;

fn main() {
    let filename = "input.txt";
    let input = day2::read_file(filename)
        .expect(&format!("couldn't open {}", filename));
    let data = day12::parse_input(&input).expect("failed to parse");
    let result1 = day12::process1(&data, "0").expect("failed to process1");
    let result2 = day12::process2(&data).expect("failed to process2");
    println!("Result 1:\n{}\nResult 2:\n{}", result1, result2);
}
