use utils::read_input;

mod part1;
mod part2;

fn main() {
    let input = read_input("input.txt");

    let sol1 = part1::solve(&input);
    println!("1: {sol1}");

    let sol2 = part2::solve(&input);
    println!("2: {sol2}");
}
