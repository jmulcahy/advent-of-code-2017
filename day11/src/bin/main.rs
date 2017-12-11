extern crate day2;
extern crate day11;

fn main() {
    let filename = "input.txt";
    let input = day2::read_file(filename)
        .expect(&format!("couldn't open {}", filename));
    let result = day11::get_shortest_dist(&input).expect("failed to parse input");
    println!("Result 1:\n{}\nResult 2:\n{}", result.0, result.1);
}
