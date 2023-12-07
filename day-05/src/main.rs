use utils::read_input;

mod part1;
mod part2;
mod part2_ranges;

fn main() {
    let input = read_input("input.txt");

    let sol1 = part1::solve(&input);
    println!("1: {sol1}");

    let sol2 = part2::solve(&input);
    println!("2: {sol2}");

    // Hyperfine results for the ranges implementation:
    //  Time (mean ± σ):      38.8 ms ±   0.5 ms    [User: 30.1 ms, System: 8.5 ms]
    //  Range (min … max):    38.0 ms …  41.4 ms    70 runs
    let sol2_ranges = part2_ranges::solve(&input);
    println!("2(ranges): {sol2_ranges}");
}
