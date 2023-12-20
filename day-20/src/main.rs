use utils::read_input;

mod part1;

fn main() {
    let input = read_input("input.txt");

    let sol1 = part1::solve(&input);
    println!("1: {sol1}");
}
