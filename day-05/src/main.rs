use utils::read_input;

mod part1;
mod part2;

fn main() {
    let input = read_input("input.txt");
    let solution = part2::solve(input);

    // this takes about 2m 17s to run in release mode
    println!("Solution: {solution}");
}
