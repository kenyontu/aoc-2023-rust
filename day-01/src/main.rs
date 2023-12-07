use utils::read_input;

pub mod part1;
pub mod part2;

fn main() {
    let input = read_input("input.txt");

    let sol1 = part1::solve(&input);
    println!("1: {sol1}");

    let sol2 = part2::solve(&input);
    println!("2: {sol2}");
}
